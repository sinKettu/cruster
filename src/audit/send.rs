use serde::{Serialize, Deserialize};
use std::{collections::HashMap, str::FromStr};

use super::AuditError;

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub(crate) struct RuleSendAction {
    id: Option<String>,
    apply: String,
    apply_cache: Option<usize>,
    repeat: Option<usize>,
    timeout_after: Option<usize>,
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
}