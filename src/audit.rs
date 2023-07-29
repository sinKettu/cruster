mod watch;
mod change;
mod send;
mod find;
mod get;

use serde_yaml as yml;
use serde::{Serialize, Deserialize};

use watch::RuleWatchAction;
use change::RuleChangeAction;
use send::RuleSendAction;
use find::RuleFindAction;
use get::RuleGetAction;

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub(crate) struct RuleMetadata {
    authors: Vec<String>,
    name: String,
    references: Vec<String>,
    tags: Vec<String>
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub(crate) struct RuleActions {
    watch: Vec<RuleWatchAction>,
    change: Vec<RuleChangeAction>,
    send: Vec<RuleSendAction>,
    find: Vec<RuleFindAction>,
    get: Vec<RuleGetAction>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub(crate) struct Rule {
    metadata: RuleMetadata,
    r#type: String,
    protocol: String,
    severity: String,
}
