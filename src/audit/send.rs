use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub(crate) struct RuleSendAction {
    id: String,
    apply: String,
    apply_cache: Option<usize>,
    repeat: Option<usize>,
    timeout_after: Option<usize>,
}
