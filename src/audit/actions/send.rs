use std::{collections::HashMap, str::FromStr, thread::sleep, time::Duration};

use bstr::ByteSlice;
use http::{header::HeaderName, HeaderValue, HeaderMap};
use log::debug;

use crate::{audit::{contexts::{active::ChangeStepResult, traits::{WithChangeAction, WithSendAction}}, types::{SingleCoordinates, SingleSendResultEntry}}, cruster_proxy::request_response::HyperRequestWrapper};
use crate::http_sender;

use self::change::{ChangeAdd, ChangeModify};

use super::*;

#[derive(Debug, Clone)]
pub(crate) struct ExecutedChangeStep {
    payloads: Vec<Arc<String>>,
    request: HyperRequestWrapper
}

impl From<&HyperRequestWrapper> for ExecutedChangeStep {
    fn from(value: &HyperRequestWrapper) -> Self {
        ExecutedChangeStep {
            payloads: vec![],
            request: value.clone()
        }
    }
}

impl ExecutedChangeStep {
    fn step(&self, payload: Option<Arc<String>>, request: HyperRequestWrapper) -> Self {
        let mut copy = self.clone();
        if let Some(payload) = payload {
            copy.payloads.push(payload);
        }
        
        copy.request = request;

        return copy;
    }

    fn request(&self) -> &HyperRequestWrapper {
        &self.request
    }

    fn payloads(&self) -> &Vec<Arc<String>> {
        &self.payloads
    }
}


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
                    let splitted: Vec<&[u8]> = new_line.splitn(2, |c| { (*c as char) == '\n' }).collect();
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
            let splitted = request.body.split(|c| { (*c as char) == '\n' }).collect::<Vec<&[u8]>>();
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

    async fn with_modify_change<'pair_lt, T>(&self, ctxt: &mut T, step: usize, modify: &ChangeModify, executed_steps: &Vec<ExecutedChangeStep>) -> Result<Vec<ExecutedChangeStep>, AuditError>
    where
        T: WithSendAction<'pair_lt> + WithChangeAction<'pair_lt> 
    {
        let placement = modify.get_placement();
        let payloads = modify.get_payloads();

        let change_to_apply = self.apply_cache.unwrap();
        let all_steps_results = &ctxt.change_results()[change_to_apply];

        let mut next_steps: Vec<ExecutedChangeStep> = Vec::with_capacity(executed_steps.len() * payloads.len());

        let coordinates = match &all_steps_results.0[step] {
            ChangeStepResult::NONE => {
                ctxt.add_send_result(Vec::default());
                debug!("SendAction - Nothing to change in initial request for this action");
                return Ok(vec![]);
            },
            ChangeStepResult::MODIFY(coordinates) => {
                coordinates
            },
            _ => {
                unreachable!("Found ChangeStep '{:#?}', but it must have type ::MODIFY or ::NONE", &all_steps_results.0[step]);
            }
        };

        // let mut results_: Vec<SingleSendResultEntry> = Vec::with_capacity(if let Some(rp) = self.repeat { payloads.len() * rp } else { payloads.len() });

        for request_with_payloads in executed_steps {
            let request = request_with_payloads.request();

            for SingleCoordinates { line: line_number, start, end } in coordinates {
                let workline = if line_number.to_owned() == 0 {
                    let method_bytes = request.method.as_bytes();
                    let path = request.get_request_path();
                    let path_bytes = path.as_bytes();
                    let version_bytes = request.version.as_bytes();
                    let request_line = [method_bytes, b" ", path_bytes, b" ", version_bytes].concat();
    
                    debug!("SendAction - changing line: {}", request_line.to_str_lossy());
                    debug!("SendAction -                {: <2$}^{: <3$}^", "", "", start.to_owned(), (end.to_owned() - start.to_owned()).saturating_sub(2));
    
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
    
                    debug!("SendAction - changing line: {}", request_line.to_str_lossy());
                    debug!("SendAction -                {: <2$}^{: <3$}^", "", "", start.to_owned(), (end.to_owned() - start.to_owned()).saturating_sub(2));
    
                    request_line
                }
                else {
                    let offset = 1 + request.headers.len();
                    let request_line = request
                        .body
                            .split(|chr| { (*chr as char) == '\n' })
                            .skip(line_number - offset)
                            .take(1)
                            .collect::<Vec<&[u8]>>()[0];
    
                    debug!("SendAction - changing line: {}", request_line.to_str_lossy());
                    debug!("SendAction -                {: <2$}^{: <3$}^", "", "", start.to_owned(), (end.to_owned() - start.to_owned()).saturating_sub(2));
    
                    request_line.to_vec()
                };
    
                for payload in payloads {
                    let (new_line, new_start, new_end) = match placement {
                        ChangeValuePlacement::AFTER => {
                            let left_line_part = &workline[0..*end];
                            let right_line_part = &workline[*end..];
    
                            (
                                [left_line_part, payload.as_bytes(), right_line_part].concat(),
                                left_line_part.len(),
                                (left_line_part.len() + payload.len()).saturating_sub(1)
                            )
                        },
                        ChangeValuePlacement::BEFORE => {
                            let left_line_part = &workline[0..*start];
                            let right_line_part = &workline[*start..];
    
                            (
                                [left_line_part, payload.as_bytes(), right_line_part].concat(),
                                left_line_part.len(),
                                (left_line_part.len() + payload.len()).saturating_sub(1)
                            )
                        },
                        ChangeValuePlacement::REPLACE => {
                            let left_line_part = &workline[0..*start];
                            let right_line_part = &workline[*end..];
                            
                            (
                                [left_line_part, payload.as_bytes(), right_line_part].concat(),
                                left_line_part.len(),
                                (left_line_part.len() + payload.len()).saturating_sub(1)
                            )
                        }
                    };
    
                    debug!("SendAction - modified line: {}", new_line.as_slice().to_str_lossy());
                    debug!("SendAction -                {: <2$}^{: <3$}^", "", "", new_start, (new_end.saturating_sub(new_start)).saturating_sub(1));
    
                    let modified_request = self.modify_request(request, new_line, line_number.to_owned())?;
                    let next_step = request_with_payloads.step(Some(payload.clone()), modified_request);
                    next_steps.push(next_step)
                }
            }
    
        }

        Ok(next_steps)
    }

    async fn with_add_change<'pair_lt, T>(&self, _ctxt: &mut T, add_list: &Vec<ChangeAdd>, executed_steps: &Vec<ExecutedChangeStep>) -> Result<Vec<ExecutedChangeStep>, AuditError> 
    where
        T: WithSendAction<'pair_lt> + WithChangeAction<'pair_lt> 
    {
        let mut next_steps: Vec<ExecutedChangeStep> = Vec::with_capacity(executed_steps.len());

        for request_with_payloads in executed_steps {
            let mut request = request_with_payloads.request().clone();

            for add in add_list {
                match add {
                    ChangeAdd::HEADER(header) => {
                        let (key, value) = header.http_header()?;
                        request.headers.insert(key, value);
                    }
                }
            }

            let next_step = request_with_payloads.step(None, request);
            next_steps.push(next_step);
        }

        Ok(next_steps)
    }

    async fn execute_requests<'pair_lt, T>(&self, ctxt: &mut T, executed_steps: Vec<ExecutedChangeStep>) -> Result<(), AuditError>
    where
        T: WithSendAction<'pair_lt> + WithChangeAction<'pair_lt> 
    {
        let mut results = vec![];
        for request_with_payloads in executed_steps {
            let mut modified_request = request_with_payloads.request().clone();
            let payloads = request_with_payloads.payloads();

            let str_body_length = modified_request.body.len().to_string();
            let _ = modified_request.headers.insert("Content-Length", HeaderValue::from_str(&str_body_length)?);

            let repeat_number = if let Some(rn) = self.repeat.as_ref() { rn.to_owned() } else { 0 };
            let timeout = if let Some(tout) = self.timeout_after.as_ref() { tout.to_owned() } else { 0 };

            let modified_request = Arc::new(modified_request);

            for j in 0..(repeat_number + 1) {
                debug!("SendAction - Sending request, repeat: {}/{}, URL: {} {} {}",
                    j + 1,
                    repeat_number + 1,
                    &modified_request.method,
                    &modified_request.uri,
                    &modified_request.version
                );

                let response = match http_sender::send_request_from_wrapper(&modified_request, 0).await {
                    Ok((response, _)) => {
                        response
                    },
                    Err(err) => {
                        let err_str = format!("Action failed on sending request (payloads={:#?}): {}", payloads, err);
                        return Err(AuditError(err_str));
                    }  
                };

                let result_entry = SingleSendResultEntry {
                    request: modified_request.clone(),
                    payloads: payloads.to_owned(),
                    response
                };

                results.push(result_entry);

                if repeat_number > 0 && timeout > 0 {
                    sleep(Duration::from_millis(timeout as u64));
                }
            }
        }

        ctxt.add_send_result(results);

        Ok(())
    }

    pub(crate) async fn exec<'pair_lt, 'rule_lt, T>(&self, ctxt: &mut T, change: &Vec<InnerChangeAction>) -> Result<(), AuditError>
    where
        T: WithSendAction<'pair_lt> + WithChangeAction<'pair_lt> 
    {
        let mut executed_steps = if let Some(request) = ctxt.initial_request() {
            vec![ExecutedChangeStep::from(request)]
        }
        else {
            unreachable!("Have empty initial request in SendAction '{:#?}'", self.id);
        };

        for (index, step) in change.iter().enumerate() {
            match step {
                InnerChangeAction::MODIFY(modify) => {
                    executed_steps = self.with_modify_change(ctxt, index, modify, &executed_steps).await?
                },
                InnerChangeAction::ADD(add) => {
                    executed_steps = self.with_add_change(ctxt, add, &executed_steps).await?
                }
            }
        }

        if ! executed_steps.is_empty() {
            self.execute_requests(ctxt, executed_steps).await
        }
        else {
            Ok(())
        }
    }
}