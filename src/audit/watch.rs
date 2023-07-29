use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub(crate) enum WatchPart {
    Method,
    Path,
    Version,
    Headers,
    Body
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub(crate) struct RuleWatchAction {
    id: Option<String>,
    part: String,
    // This field will store more convinient representation of part after first check
    part_cache: Option<WatchPart>,
    pattern: String
}