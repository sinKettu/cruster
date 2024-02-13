use std::collections::HashMap;

use regex::Regex;

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

        if let Err(e) = Regex::new(&self.pattern) {
            let err_str = format!("Given pattern - '{}' - cannot be parsed as regex", &self.pattern);
            return Err(AuditError(err_str));
        }

        Ok(())
    }
}
