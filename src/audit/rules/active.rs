use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::audit::actions::{RuleChangeAction, RuleFindAction, RuleGetAction, RuleSendAction, RuleWatchAction};


#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub(crate) struct ActiveRule {
    pub(crate) watch: Vec<RuleWatchAction>,
    pub(crate) change: Vec<RuleChangeAction>,
    pub(crate) send: Vec<RuleSendAction>,
    pub(crate) find: Vec<RuleFindAction>,
    pub(crate) get: Option<Vec<RuleGetAction>>,

    // These are "service" fields, to be used by cruster
    pub(crate) watch_ref: Option<HashMap<String, usize>>,
    pub(crate) change_ref: Option<HashMap<String, usize>>,
    pub(crate) send_ref: Option<HashMap<String, usize>>,
    pub(crate) find_ref: Option<HashMap<String, usize>>,
}

