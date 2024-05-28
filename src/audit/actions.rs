pub(super) mod change;
pub(super) mod find;
pub(super) mod send;
pub(super) mod watch;
pub(super) mod get;

use std::sync::Arc;

use serde::{Serialize, Deserialize};

use self::change::InnerChangeAction;

use super::AuditError;


// Used to parse string watch_id to speed up future operations
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub(crate) struct WatchId {
    pub(crate) id: usize,
    pub(crate) group_name: Option<String>
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
#[serde(rename_all = "lowercase")]
pub(crate) enum ChangeValuePlacement {
    BEFORE,
    AFTER,
    REPLACE
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub(crate) struct RuleChangeAction {
    id: Option<String>,
    watch_id: String,
    r#type: InnerChangeAction,

    watch_id_cache: Option<WatchId>, // This field will store more convinient representation of watch_id after first check
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub(crate) struct RuleFindAction {
    id: Option<String>,

    // look_for: String,
    // // This field stores more convinient representation of look_for after first check
    // look_for_cache: Option<find::LookFor>,

    exec: Vec<find::ExecutableExpression>,

    required_send_actions: Option<Vec<usize>>,
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
#[serde(rename_all = "lowercase")]
pub(crate) enum ExtractionMode {
    LINE,
    MATCH,
    GROUP(String)
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub(crate) enum ExtractionModeByPart {
    REQUEST(ExtractionMode),
    RESPONSE(ExtractionMode)
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub(crate) struct RuleGetAction {
    from: String,
    from_cache: Option<usize>,

    if_succeed: String,
    if_succeed_cache: Option<usize>,

    extract: ExtractionModeByPart,

    pattern: String
}


