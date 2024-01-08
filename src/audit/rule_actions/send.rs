use std::{collections::HashMap, str::{FromStr, Bytes}};

use bstr::ByteSlice;
use http::{header::HeaderName, HeaderValue, HeaderMap};
use nom::AsChar;

use crate::cruster_proxy::request_response::HyperRequestWrapper;

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

    fn modify_request(&self, request: &HyperRequestWrapper, new_line: Vec<u8>, (ln, start, end): ReqResCoordinates) -> Result<HyperRequestWrapper, AuditError> {
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
            let mut splitted = request.body.split(|c| {c.as_char() == '\n'}).collect::<Vec<&[u8]>>();
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

    pub(crate) fn exec(&self, change_results: &Vec<&Vec<ReqResCoordinates>>, placement: ChangeValuePlacement, payloads: &Vec<String>, request: &HyperRequestWrapper) -> Result<(), AuditError> {
        let change_to_apply = self.apply_cache.unwrap();

        for (line_number, start, end) in change_results[change_to_apply] {
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

            for payload in payloads {
                match placement {
                    ChangeValuePlacement::AFTER => {
                        let left_line_part = &workline[0..=*end];
                        let right_line_part = &workline[*end+1..];
                        let new_line = [left_line_part, payload.as_bytes(), right_line_part].concat();


                    },
                    ChangeValuePlacement::BEFORE => {
                        todo!()
                    },
                    ChangeValuePlacement::REPLACE => {
                        todo!()
                    }
                }
            }
        }


        todo!()
    }
}