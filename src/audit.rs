pub(crate) mod actions;
pub(crate) mod load_rule;
pub(crate) mod execution;
pub(crate) mod rules;
pub(crate) mod contexts;
pub(crate) mod types;
pub(crate) mod result;

use std::{collections::HashMap, fmt::{Debug, Display}, str::FromStr};
use serde::{Serialize, Deserialize};

use actions::{
    RuleChangeAction,
    RuleFindAction,
    RuleSendAction,
    RuleWatchAction,
    RuleGetAction
};

use self::{rules::{active::ActiveRule, passive::PassiveRule}, types::{SerializableSendResultEntry, SingleSendResultEntry}};

pub(crate) struct AuditError(String);

impl FromStr for AuditError {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(AuditError(s.to_string()))
    }
}

impl Display for AuditError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.as_str())
    }
}

impl Debug for AuditError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AuditError {{ String({}) }}", &self.0)
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub(crate) struct RuleMetadata {
    authors: Vec<String>,
    name: String,
    references: Vec<String>,
    tags: Vec<String>
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub(crate) struct RuleActions {
    watch: Option<Vec<RuleWatchAction>>,
    change: Option<Vec<RuleChangeAction>>,
    send: Option<Vec<RuleSendAction>>,
    find: Option<Vec<RuleFindAction>>,
    get: Option<Vec<RuleGetAction>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub(crate) enum RuleType {
    Active(ActiveRule),
    Passive(PassiveRule),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub(crate) enum RuleSeverity {
    Info,
    Low,
    Medium,
    High
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub(crate) enum RuleByProtocal {
    Http(RuleType),
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub(crate) struct Rule {
    // These are "working" fields, to be used by users
    metadata: RuleMetadata,
    severity: RuleSeverity,
    id: String,
    rule: RuleByProtocal,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub(crate) struct RuleResult {
    rule_id: String,
    pair_index: usize,
    severity: RuleSeverity,
    findings: HashMap<String, (Vec<String>, Vec<SerializableSendResultEntry>)>,
    initial_request: String,
    initial_response: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub(crate) enum RuleFinalState {
    Skipped(String),
    Finished(Option<RuleResult>),
    Failed(String)
}
