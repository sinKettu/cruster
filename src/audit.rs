mod rule_actions;
pub(crate) mod load_rule;
pub(crate) mod execution;
pub(crate) mod rules;
pub(crate) mod rule_contexts;
pub(crate) mod types;

use std::{fmt::{Display, Debug}, str::FromStr};
use serde::{Serialize, Deserialize};

use rule_actions::{
    RuleChangeAction,
    RuleFindAction,
    RuleSendAction,
    RuleWatchAction,
    RuleGetAction
};

use self::rule_contexts::traits::RuleExecutionContext;

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

impl From<nom::error::Error<&str>> for AuditError {
    fn from(value: nom::error::Error<&str>) -> Self {
        AuditError(value.to_string())
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

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub(crate) enum RuleType {
    Active,
    Passive,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub(crate) struct Rule {
    // These are "working" fields, to be used by users
    metadata: RuleMetadata,
    r#type: RuleType,
    protocol: String,
    severity: String,
    id: String,
    max_redirects: usize,
    rule: RuleActions,
    // These are "service" fields, to be used by cruster
    watch_ref: Option<std::collections::HashMap<String, usize>>,
    change_ref: Option<std::collections::HashMap<String, usize>>,
    send_ref: Option<std::collections::HashMap<String, usize>>,
    find_ref: Option<std::collections::HashMap<String, usize>>,
}

pub(crate) struct RuleResult {
    rule_id: String
}

pub(crate) enum RuleFinalState {
    Skipped(String),
    Finished(RuleResult),
    Failed(String)
}
