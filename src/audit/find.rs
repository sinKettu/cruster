use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub(crate) enum LookFor {
    ANY,
    ALL
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub(crate) struct RuleFindAction {
    id: String,

    look_for: String,
    // This field will store more convinient representation of look_for after first check
    look_for_cache: Option<LookFor>,

    expressions: Vec<String>
}