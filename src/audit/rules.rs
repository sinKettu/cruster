pub(crate) mod active;
pub(crate) mod passive;

use std::{fmt::Display, sync::Arc};

use self::active::ActiveRule;

use super::{actions::{change::InnerChangeAction, ChangeValuePlacement, RuleChangeAction, RuleFindAction}, AuditError, Rule, RuleByProtocal, RuleType};


// TODO: Need also check for indexes bounds in check_up() methods
impl Rule {
    fn make_error<T: Display>(&self, possible_details: Option<T>) -> AuditError {
        if let Some(details) = possible_details {
            AuditError(
                format!("Rule {} has the problem. {}", self.id.as_str(), details)
            )
        }
        else {
            AuditError(
                format!("Rule {} has an undefined error!", self.id.as_str())
            )
        }
    }

    pub(crate) fn check_up(&mut self) -> Result<(), AuditError> {
        match &mut self.rule {
            RuleByProtocal::Http(rule_type) => {
                match rule_type {
                    RuleType::Active(actions) => {
                        actions.check_up()
                    },
                    RuleType::Passive(actions) => {
                        actions.check_up()
                    }
                }
            }
        }
    }

    pub(crate) fn http_active_rule(&self) -> Result<&ActiveRule, AuditError> {
        match &self.rule {
            RuleByProtocal::Http(rule_type) => {
                match rule_type {
                    RuleType::Active(rule) => {
                        Ok(rule)
                    },
                    RuleType::Passive(_) => {
                        let err_str = format!("trying to work with '{}' like active rule, but it's passive", self.get_id());
                        Err(AuditError(err_str))
                    }
                }
            }
        }
    }

    pub(crate) fn http_active_rule_mut(&mut self) -> Result<&mut ActiveRule, AuditError> {
        let id = self.get_id().to_string();
        match &mut self.rule {
            RuleByProtocal::Http(rule_type) => {
                match rule_type {
                    RuleType::Active(rule) => {
                        Ok(rule)
                    },
                    RuleType::Passive(_) => {
                        let err_str = format!("trying to work with '{}' like active rule, but it's passive", id);
                        Err(AuditError(err_str))
                    }
                }
            }
        }
    }

    fn get_change_action_by_index(&self, index: usize) -> Result<&RuleChangeAction, AuditError> {
        let rule = self.http_active_rule()?;
        let Some(change) = rule.change.get(index) else {
            let err_str = format!("Tried to access change action with index {} in rule '{}', but it contains only {} change actions", index, self.id.as_str(), rule.change.len());
            return Err(AuditError(err_str));
        };

        return Ok(change);
    }

    pub(crate) fn get_change_inner_action_by_index(&self, index: usize) -> Result<&InnerChangeAction, AuditError> {
        let change_action = self.get_change_action_by_index(index)?;
        Ok(change_action.get_inner_action())
    }

    pub(crate) fn get_change_placement_by_index(&self, index: usize) -> Result<ChangeValuePlacement, AuditError> {
        let change = self.get_change_action_by_index(index)?;
        change.get_placement()
    }

    pub(crate) fn get_payloads_by_index(&self, index: usize) -> Result<&Vec<Arc<String>>, AuditError> {
        let change = self.get_change_action_by_index(index)?;
        change.get_payloads()
    }

    // pub fn get_send_actions_number(&self) -> Result<usize, AuditError> {
    //     let rule = self.http_active_rule()?;
    //     Ok(rule.send.len())
    // }

    // pub fn get_watch_actions_number(&self) -> Result<usize, AuditError> {
    //     let rule = self.http_active_rule()?;
    //     Ok(rule.watch.len())
    // }
    
    pub(crate) fn get_id(&self) -> &str {
        return &self.id;
    }

    // pub(crate) fn watch_actions(&self) -> Result<&Vec<RuleWatchAction>, AuditError> {
    //     let rule = self.http_active_rule()?;
    //     Ok(&rule.watch)
    // }

    pub(crate) fn get_find_actions(&self) -> Result<&Vec<RuleFindAction>, AuditError> {
        // let rule = self.http_active_rule()?;
        let find_actions = match &self.rule {
            RuleByProtocal::Http(rule_type) => {
                match rule_type {
                    RuleType::Active(actions) => {
                        &actions.find
                    },
                    RuleType::Passive(actions) => {
                        &actions.find
                    }
                }
            }
        };

        Ok(find_actions)
    }

    pub(crate) fn get_find_action_str_id(&self, index: usize) -> Result<String, AuditError> {
        if index >= self.get_find_actions()?.len() {
            let err_str = format!("Index ({}) of find action is out of bounds", index);
            return Err(AuditError(err_str));
        }

        if let Some(str_id) = self.get_find_actions()?[index].get_id() {
            Ok(str_id)
        }
        else {
            Ok(index.to_string())
        }
    }
}
