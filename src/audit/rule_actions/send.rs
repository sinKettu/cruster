use std::{collections::HashMap, str::FromStr, thread::sleep, time::Duration};

use bstr::ByteSlice;
use http::{header::HeaderName, HeaderValue, HeaderMap};
use nom::AsChar;

use crate::{audit::{rule_contexts::traits::RuleExecutionContext, types::{PayloadsTests, SendActionResultsPerPatternEntry, SingleCoordinates, SingleSendActionResult}}, cruster_proxy::request_response::HyperRequestWrapper};
use crate::http_sender;

use super::*;

impl RuleSendAction {
    pub(crate) fn check_up(&mut self, possible_change_ref: Option<&HashMap<String, usize>>) -> Result<(), AuditError> {
        if let Ok(num_change_id) = self.apply.parse::<usize>() {
            self.apply_cache = Some(num_change_id);
        }
        else {
            if let Some(change_ref) = possible_change_ref {
                if let Some(num_change_id) = change_ref.get(&self.apply) {
                    self.apply_cache = Some(num_change_id.to_owned())
                }
                else {
                    return Err(
                        AuditError::from_str(format!("watch action with id '{}' is not found", &self.apply).as_str()).unwrap()
                    );
                }
            }
            else {
                return Err(
                    AuditError::from_str(format!("watch action with id '{}' cannot be found", &self.apply).as_str()).unwrap()
                );
            }
        }

        Ok(())
    }

    pub(crate) fn get_id(&self) -> Option<String> {
        self.id.clone()
    }

    pub(crate) fn get_apply_id(&self) -> usize {
        self.apply_cache.as_ref().unwrap().to_owned()
    }

    fn modify_request(&self, request: &HyperRequestWrapper, new_line: Vec<u8>, ln: usize) -> Result<HyperRequestWrapper, AuditError> {
        let headers_number = request.headers.len();
        
        if ln == 0 {
            let Ok(new_line) = new_line.to_str() else {
                let err_str = format!("Cannot encode to UTF-8: {}", new_line.to_str_lossy());
                    return Err(AuditError(err_str));
            };

            let splitted: Vec<&str> = new_line.split(' ').collect();
            
            if splitted.len() < 3 {
                let err_str = format!(
                    "Cannot apply {} change: format error in request's first line: {}",
                    self.apply_cache.unwrap(),
                    &new_line
                );

                return Err(AuditError(err_str));
            }

            let new_method = splitted[0];
            let new_version = splitted[splitted.len() - 1];

            let amount_to_take = splitted.len() - 2;
            // TODO: change to _intersperse_ someday
            let mut new_path = String::with_capacity(new_line.len());
            for (index, path_part) in splitted.into_iter().skip(1).take(amount_to_take).enumerate() {
                if index == 0 {
                    new_path.push_str(path_part);
                }
                else {
                    new_path.push_str(" ");
                    new_path.push_str(path_part);
                }
            }

            let mut new_request = request.clone();
            new_request.method = new_method.to_string();
            new_request.uri = format!("{}{}{}", request.get_scheme(), request.get_hostname(), new_path);
            new_request.version = new_version.to_string();

            return Ok(new_request);
        }
        else if ln > 0 && ln <= headers_number {
            let mut new_headers: HeaderMap<HeaderValue> = HeaderMap::with_capacity(headers_number);
            for (index, (k, v)) in request.headers.iter().enumerate() {
                if index == ln - 1 {
                    let splitted: Vec<&[u8]> = new_line.splitn(2, |c| { c.as_char() == ':' }).collect();
                    let Ok(hn) = HeaderName::from_bytes(splitted[0]) else {
                        let err_str = format!("Cannot apply {} change: issue with converting '{}' into header name", self.apply_cache.unwrap(), new_line.to_str_lossy());
                        return Err(AuditError(err_str))
                    };

                    // Cutting trailing whitespace at beggining
                    let tmp = splitted[1];
                    let tmp = &tmp[1..];

                    let Ok(hv) = HeaderValue::from_bytes(tmp) else {
                        let err_str = format!("Cannot apply {} change: issue with converting '{}' into header value", self.apply_cache.unwrap(), new_line.to_str_lossy());
                        return Err(AuditError(err_str))
                    };

                    new_headers.append(hn, hv);
                }
                else {
                    new_headers.append(k, v.clone());
                }
            }

            let mut new_request = request.clone();
            new_request.headers = new_headers;

            return Ok(new_request)
        }
        else {
            let splitted = request.body.split(|c| {c.as_char() == '\n'}).collect::<Vec<&[u8]>>();
            let index = ln - headers_number - 1;
            if index >= splitted.len() {
                let err_str = format!("Cannot apply {} change, because it's index is out of bounds of the request", self.apply_cache.unwrap());
                return Err(AuditError(err_str));
            }

            let mut new_body: Vec<u8> = Vec::with_capacity(request.body.len());
            for (i, body_part) in splitted.into_iter().enumerate() {
                if i != 0 {
                    new_body.push(b'\n');
                }

                if i != index {
                    new_body.extend(body_part);

                }
                else {
                    new_body.extend(new_line.iter());
                }
            }

            let mut new_request = request.clone();
            new_request.body = new_body;

            return Ok(new_request);
        }
    }

    pub(crate) async fn exec<'pair_lt, 'rule_lt, T: RuleExecutionContext<'pair_lt, 'rule_lt>>(&self, ctxt: &mut T, placement: ChangeValuePlacement, payloads: &'rule_lt Vec<String>) -> Result<(), AuditError> {
        let change_to_apply = self.apply_cache.unwrap();
        let coordinates = &ctxt.change_results()[change_to_apply];
        let request = ctxt.initial_request().unwrap();

        // First level - one per pattern entry
        // Second level - one per payload value
        // third level - one per every repeat
        let mut results = SendActionResultsPerPatternEntry::with_capacity(coordinates.len());
        for SingleCoordinates { line: line_number, start, end} in coordinates {
            let workline = if line_number.to_owned() == 0 {
                let method_bytes = request.method.as_bytes();
                let path = request.get_request_path();
                let path_bytes = path.as_bytes();
                let version_bytes = request.version.as_bytes();
                let request_line = [method_bytes, b" ", path_bytes, b" ", version_bytes].concat();

                request_line
            }
            else if line_number.to_owned() >= 1 && line_number.to_owned() <= request.headers.len() {
                let (key, value) = request
                    .headers
                        .iter()
                        .skip(line_number - 1)
                        .take(1)
                        .collect::<Vec<(&HeaderName, &HeaderValue)>>()[0];

                let request_line = [key.as_str().as_bytes(), b": ", value.as_bytes()].concat();
                request_line
            }
            else {
                let offset = 1 + request.headers.len();
                let request_line = request
                    .body
                        .split(|chr| { chr.as_char() == '\n' })
                        .skip(line_number - offset)
                        .take(1)
                        .collect::<Vec<&[u8]>>()[0];

                request_line.to_vec()
            };

            let mut second_level_results: PayloadsTests = PayloadsTests::with_capacity(payloads.len());
            for payload in payloads {
                let (new_line, new_start, new_end) = match placement {
                    ChangeValuePlacement::AFTER => {
                        let left_line_part = &workline[0..=*end];
                        let right_line_part = &workline[*end+1..];

                        (
                            [left_line_part, payload.as_bytes(), right_line_part].concat(),
                            left_line_part.len(),
                            (left_line_part.len() + payload.len()).saturating_sub(1)
                        )
                    },
                    ChangeValuePlacement::BEFORE => {
                        let left_line_part = &workline[0..=*start];
                        let right_line_part = &workline[*start+1..];

                        (
                            [left_line_part, payload.as_bytes(), right_line_part].concat(),
                            left_line_part.len(),
                            (left_line_part.len() + payload.len()).saturating_sub(1)
                        )
                    },
                    ChangeValuePlacement::REPLACE => {
                        let left_line_part = &workline[0..=*start];
                        let right_line_part = &workline[*end+1..];
                        
                        (
                            [left_line_part, payload.as_bytes(), right_line_part].concat(),
                            left_line_part.len(),
                            (left_line_part.len() + payload.len()).saturating_sub(1)
                        )
                    }
                };

                let modified_request = self.modify_request(request, new_line, line_number.to_owned())?;
                let repeat_number = if let Some(rn) = self.repeat.as_ref() { rn.to_owned() } else { 0 };
                let timeout = if let Some(tout) = self.timeout_after.as_ref() { tout.to_owned() } else { 0 };

                let mut third_level_results: SingleSendActionResult = SingleSendActionResult {
                    request_sent: modified_request,
                    positions_changed: SingleCoordinates {
                        line: line_number.to_owned(),
                        start: new_start,
                        end: new_end
                    },
                    responses_received: Vec::with_capacity(repeat_number + 1)
                };

                for _ in 0..(repeat_number + 1) {
                    let response = match http_sender::send_request_from_wrapper(&third_level_results.request_sent, 0).await {
                        Ok((response, _)) => {
                            response
                        },
                        Err(err) => {
                            let err_str = format!("Action failed on sending request: {}", err);
                            return Err(AuditError(err_str));
                        }  
                    };

                    third_level_results.responses_received.push(response);

                    if repeat_number > 0 && timeout > 0 {
                        sleep(Duration::from_millis(timeout as u64));
                    }
                }

                second_level_results.insert(payload.as_str(), third_level_results);
            }

            results.push(second_level_results);
        }

        ctxt.add_send_result(results);

        Ok(())
    }
}