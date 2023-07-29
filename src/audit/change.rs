use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub(crate) struct WatchId {
    id: usize,
    group_name: Option<String>
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub(crate) enum ChangeValuePlacement {
    BEFORE,
    AFTER,
    REPLACE
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub(crate) struct RuleChangeAction {
    id: String,

    watch_id: String,
    // This field will store more convinient representation of watch_id after first check
    watch_id_cache: Option<WatchId>,

    placement: String,
    // This field will store more convinient representation of placement after first check
    placement_cache: Option<ChangeValuePlacement>,

    values: Vec<String>,
}