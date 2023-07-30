mod watch;
mod change;
mod send;
mod find;
mod get;

use std::{fmt::Display, str::FromStr};

use serde_yaml as yml;
use serde::{Serialize, Deserialize};

use watch::RuleWatchAction;
use change::RuleChangeAction;
use send::RuleSendAction;
use find::RuleFindAction;
use get::RuleGetAction;

pub(crate) struct AuditError(String);

trait AuditErrorTrait {}

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

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub(crate) struct RuleMetadata {
    authors: Vec<String>,
    name: String,
    references: Vec<String>,
    tags: Vec<String>
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub(crate) struct RuleActions {
    watch: Option<Vec<RuleWatchAction>>,
    change: Option<Vec<RuleChangeAction>>,
    send: Option<Vec<RuleSendAction>>,
    find: Option<Vec<RuleFindAction>>,
    get: Option<Vec<RuleGetAction>>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub(crate) struct Rule {
    // These are "working" fields, to be used by users
    metadata: RuleMetadata,
    r#type: String,
    protocol: String,
    severity: String,
    id: String,
    rule: RuleActions,
    // These are "service" fields, to be used by cruster
    watch_ref: Option<std::collections::HashMap<String, usize>>,
    change_ref: Option<std::collections::HashMap<String, usize>>,
    send_ref: Option<std::collections::HashMap<String, usize>>,
}

// TODO: Need also check for indexes bounds in check_up() methods
impl Rule {
    fn make_error<T: Display>(&self, possible_details: Option<T>) -> AuditError {
        if let Some(details) = possible_details {
            AuditError(
                format!("Rule {} has the problem: {}", self.id.as_str(), details)
            )
        }
        else {
            AuditError(
                format!("Rule {} hash an undefined error!", self.id.as_str())
            )
        }
    }

    pub(crate) fn check_up(&mut self) -> Result<(), AuditError> {
        // Check that type of rule is known
        match self.r#type.to_lowercase().as_str() {
            // Active rules require the following actions:
            // WATCH, CHANGE, SEND, FIND
            "active" => {
                if ! (
                    self.rule.watch.is_some() 
                    && self.rule.change.is_some()
                    && self.rule.send.is_some()
                    && self.rule.find.is_some()
                ) {
                    return Err(self.make_error(Some("active rule requires actions WATCH, CHANGE, SEND and FIND")));
                }
            },
            // Passive rule require only FIND action
            "passive" => {
                if self.rule.find.is_none() {
                    return Err(self.make_error(Some("passive rule requires FIND action")));
                }
            },
            _ => {
                return Err(
                    self.make_error(
                        Some(format!("unsupported type '{}'", &self.r#type))
                    )
                );
            }
        }

        // Check that protocol is known
        match self.protocol.to_lowercase().as_str() {
            "http" => {},
            "websocket" => {
                todo!("WebSocket rules are unsupported for now")
            },
            _ => {
                return Err(
                    self.make_error(
                        Some(format!("unsupported protocol '{}'", &self.protocol))
                    )
                );
            }
        }

        // Validate .severity field and force lowercase
        self.severity = self.severity.to_lowercase();
        match self.severity.as_str() {
            "info" => {},
            "low" => {},
            "medium" => {},
            "high" => {},
            _ => {
                return Err(
                    self.make_error(
                        Some(format!("unknown severity '{}'", &self.severity))
                    )
                );
            }
        }

        // Check variable values in Watch struct and fill .watch_ref
        if let Some(watch_actions) = self.rule.watch.as_mut() {
            self.watch_ref = Some(std::collections::HashMap::default());
            for (index, watch_action) in watch_actions.iter_mut().enumerate() {
                if let Err(err) = watch_action.check_up() {
                    return Err(self.make_error(Some(err)));
                }
                
                if let Some(watch_id) = watch_action.get_id() {
                    self.watch_ref
                        .as_mut()
                        .unwrap()
                        .insert(watch_id, index);
                }
            }
        }
        

        // Check variable values and references in Change struct and fill .change_ref
        if let Some(change_actions) = self.rule.change.as_mut() {
            self.change_ref = Some(std::collections::HashMap::default());
            for (index, change_action) in change_actions.iter_mut().enumerate() {
                if let Err(err) = change_action.check_up(self.watch_ref.as_ref().unwrap()) {
                    return Err(self.make_error(Some(err)));
                }

                if let Some(change_id) = change_action.get_id() {
                    self.change_ref
                        .as_mut()
                        .unwrap()
                        .insert(change_id, index);
                }
            }
        }
        

        Ok(())
    }
}
