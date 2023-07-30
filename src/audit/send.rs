use serde::{Serialize, Deserialize};
use std::collections::HashMap;

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
    pub(crate) fn check_up(&mut self, change_ref: Option<&HashMap<String, usize>>) -> Result<(), AuditError> {
        todo!()
    }
}