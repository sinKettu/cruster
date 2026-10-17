// YAML rules are frozen; keep their implementation until the redesign resumes.
#[allow(dead_code)]
pub(crate) mod actions;
pub(crate) mod load_rule;
pub(crate) mod execution;
#[allow(dead_code)] // Frozen YAML rule validation and accessors.
pub(crate) mod rules;
#[allow(dead_code, unused_imports)] // Frozen YAML execution contexts.
pub(crate) mod contexts;
pub(crate) mod types;
pub(crate) mod result;
pub(crate) mod python;
pub(crate) 

use std::{collections::HashMap, fmt::{Debug, Display}, str::FromStr};
use serde::{Serialize, Deserialize};

use actions::{
    RuleChangeAction,
    RuleFindAction,
    RuleSendAction,
    RuleWatchAction,
    RuleGetAction
};

use self::{rules::{active::ActiveRule, passive::PassiveRule}, types::SerializableSendResultEntry};

#[derive(Debug, Clone)]
pub(crate) enum AuditRule {
    #[allow(dead_code)] // YAML rule loading is temporarily disabled.
    Yaml(Rule),
    Python(python::PythonRule),
}

impl AuditRule {
    pub(crate) fn get_id(&self) -> &str {
        match self {
            Self::Yaml(rule) => rule.get_id(),
            Self::Python(rule) => rule.get_id(),
        }
    }

    pub(crate) async fn execute(&self, pair: std::sync::Arc<crate::http_storage::RequestResponsePair>) -> (RuleFinalState, Vec<python::PythonRuleLog>) {
        match self {
            Self::Yaml(_) => todo!("YAML audit rules are temporarily disabled while their design is being reconsidered"),
            Self::Python(rule) => rule.execute(pair).await,
        }
    }

    pub(crate) async fn execute_many_stream(
        &self,
        pairs: Vec<std::sync::Arc<crate::http_storage::RequestResponsePair>>,
        tx: tokio::sync::mpsc::UnboundedSender<(usize, RuleFinalState, Vec<python::PythonRuleLog>)>,
    ) -> Result<(), String> {
        match self {
            Self::Yaml(_) => todo!("YAML audit rules are temporarily disabled while their design is being reconsidered"),
            Self::Python(rule) => rule.execute_many_stream(pairs, tx).await,
        }
    }
}

pub(crate) struct AuditError(String);

impl FromStr for AuditError {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(AuditError(s.to_string()))
    }
}

// impl Display for AuditError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.0.as_str())
//     }
// }

impl Debug for AuditError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AuditError {{ String({}) }}", &self.0)
    }
}

impl<T> From<T> for AuditError where T: Display {
    fn from(value: T) -> Self {
        AuditError(format!("{}", value))
    }
}

impl Into<String> for AuditError {
    fn into(self) -> String {
        self.0
    }
}

impl AuditError {
    pub(crate) fn to_string(&self) -> String {
        self.0.clone()
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub(crate) struct RuleMetadata {
    about: String,
    authors: Vec<String>,
    name: String,
    references: Vec<String>,
    tags: Vec<String>
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[allow(dead_code)] // Frozen YAML action schema.
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
    id: usize,
    rule_id: String,
    about: String,
    protocol: String,
    r#type: String,
    pair_index: usize,
    severity: RuleSeverity,
    pub(crate) findings: HashMap<String, (Vec<String>, Vec<SerializableSendResultEntry>)>,
    initial_request: String,
    initial_response: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub(crate) enum RuleFinalState {
    Skipped(String),
    Finished(Option<RuleResult>),
    Failed(String)
}
