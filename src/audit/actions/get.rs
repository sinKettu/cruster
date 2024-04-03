use std::collections::HashMap;

use bstr::ByteSlice;
use log::debug;
use regex::bytes::Regex;

use crate::audit::{contexts::traits::{WithFindAction, WithGetAction}, types::PayloadsTests};
use crate::cruster_proxy::request_response::extract::ExtractFromHTTPPartByRegex;

use super::*;

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub(super) enum ExtractModifier {
    LINE,
    MATCH,
    Group(String)
}

impl RuleGetAction {
    pub(crate) fn check_up(&mut self, possible_find_ref: Option<&HashMap<String, usize>>, possible_send_ref: Option<&HashMap<String, usize>>) -> Result<(), AuditError> {
        self.from_cache = if let Ok(str_from_id) = self.from.parse::<usize>() {
            Some(str_from_id)
        } 
        else {
            if let Some(refer) = possible_send_ref {
                if let Some(index) = refer.get(&self.from) {
                    Some(index.to_owned())
                }
                else {
                    let err_str = format!("Found send action id '{}' in get action, but cannot find action with this id", self.from.as_str());
                    return Err(AuditError(err_str));    
                }
            }
            else {
                let err_str = format!("Found send action id '{}' in get action, but cannot find action with this id", self.from.as_str());
                return Err(AuditError(err_str));
            }
        };

        self.if_succeed_cache = if let Ok(str_find_id) = self.if_succeed.parse::<usize>() {
            Some(str_find_id)
        }
        else {
            if let Some(refer) = possible_find_ref {
                if let Some(index) = refer.get(&self.if_succeed) {
                    Some(index.to_owned())
                }
                else {
                    let err_str = format!("Found find action id '{}' in get, but cannot find action with this id", self.if_succeed.as_str());
                    return Err(AuditError(err_str));    
                }
            }
            else {
                let err_str = format!("Found find action id '{}' in get, but cannot find the action with this id", self.if_succeed.as_str());
                return Err(AuditError(err_str));
            }
        };

        if let Err(_) = Regex::new(&self.pattern) {
            let err_str = format!("Given pattern - '{}' - cannot be parsed as regex", &self.pattern);
            return Err(AuditError(err_str));
        }

        Ok(())
    }

    pub(crate) fn exec<'pair_lt, 'rule_lt, T>(&self, ctxt: &mut T) -> Result<(), AuditError>
    where
        T: WithFindAction<'pair_lt> + WithGetAction<'pair_lt>
    {
        let find_id = self.if_succeed_cache.unwrap();
        if ! ctxt.find_action_secceeded(find_id) {
            debug!("GetAction - find action with id {} did not succeeded, action finished", find_id);
            ctxt.add_empty_result(find_id);
            return Ok(())
        }

        // Checks are done before
        let pattern = Regex::new(&self.pattern).unwrap();

        let send_data: &Vec<PayloadsTests> = ctxt.get_pair_by_id(find_id)?;
        for accordance in send_data {
            for (_, send_result) in accordance {
                let request = &send_result.request_sent;
                let responses = &send_result.responses_received;

                match &self.extract {
                    ExtractionModeByPart::REQUEST(mode) => {
                        let possible_extracted_data = request.extract(&pattern, mode);
                        if let Some(extracted_data) = possible_extracted_data {
                            debug!("GetAction - extracted data from request: {}", extracted_data.as_slice().to_str_lossy());
                            ctxt.add_get_result(find_id, extracted_data);
                            return Ok(());
                        }
                    },
                    ExtractionModeByPart::RESPONSE(mode) => {
                        for response in responses {
                            let possible_extracted_data = response.extract(&pattern, mode);
                            if let Some(extracted_data) = possible_extracted_data {
                                debug!("GetAction - extracted data from response: {}", extracted_data.as_slice().to_str_lossy());
                                ctxt.add_get_result(find_id, extracted_data);
                                return Ok(());
                            }
                        }
                    }
                }


            }
        }

        debug!("GetAction - no data extracted");
        ctxt.add_empty_result(find_id);
        Ok(())
    }
}
