pub(super) mod change;
pub(super) mod find;
pub(super) mod send;
pub(super) mod watch;
pub(super) mod get;

use serde::{Serialize, Deserialize};

use super::AuditError;
use super::expressions::functions::Function;


// Used to parse string watch_id to speed up future operations
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub(crate) struct WatchId {
    id: usize,
    group_name: Option<String>
}

impl WatchId {
    pub(crate) fn new(id: usize, group_name: Option<String>) -> Self {
        WatchId {
            id,
            group_name
        }
    }
}

// Found value can be mutated in three ways: payload may be placed before, after or instead of the value
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub(crate) enum ChangeValuePlacement {
    BEFORE,
    AFTER,
    REPLACE
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub(crate) struct RuleChangeAction {
    id: Option<String>,

    watch_id: String,
    // This field will store more convinient representation of watch_id after first check
    watch_id_cache: Option<WatchId>,

    placement: String,
    // This field will store more convinient representation of placement after first check
    placement_cache: Option<ChangeValuePlacement>,

    values: Vec<String>,
}


#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub(crate) enum LookFor {
    ANY,
    ALL
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub(crate) struct RuleFindAction {
    id: Option<String>,

    look_for: String,
    // This field stores more convinient representation of look_for after first check
    look_for_cache: Option<LookFor>,

    expressions: Vec<String>,
    // This field stores parsed expressions in a shape convinient to execute
    parsed_expressions: Option<Vec<Function>>
}


#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub(crate) struct RuleSendAction {
    id: Option<String>,
    apply: String,
    apply_cache: Option<usize>,
    repeat: Option<usize>,
    timeout_after: Option<usize>,
}


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


#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub(crate) enum ExtractionMode {
    LINE,
    MATCH,
    GROUP(String)
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub(crate) struct RuleGetAction {
    find_id: String,
    // This field will store more convinient representation of find_id after first check
    find_id_cache: Option<usize>,

    extract: String,
    // This field will store more convinient representation of extract after first check
    extract_cache: Option<ExtractionMode>,

    pattern: String
}


