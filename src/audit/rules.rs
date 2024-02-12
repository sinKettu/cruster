use std::fmt::Display;

use super::{rule_actions::{ChangeValuePlacement, RuleWatchAction}, AuditError, Rule, RuleType};


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
        // Check that type of rule is known
        match self.r#type {
            // Active rules require the following actions:
            // WATCH, CHANGE, SEND, FIND
            RuleType::Active => {
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
            RuleType::Passive => {
                if self.rule.find.is_none() {
                    return Err(self.make_error(Some("passive rule requires FIND action")));
                }
            },
            _ => {
                return Err(
                    self.make_error(
                        Some("unsupported rule type")
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


        // Check variable values and references in SEND struct and fill .send_ref
        if let Some(send_actions) = self.rule.send.as_mut() {
            self.send_ref = Some(std::collections::HashMap::default());
            for (index, send_action) in send_actions.iter_mut().enumerate() {
                if let Err(err) = send_action.check_up(self.change_ref.as_ref()) {
                    return Err(self.make_error(Some(err)));
                }

                if let Some(send_id) = send_action.get_id() {
                    self.send_ref
                        .as_mut()
                        .unwrap()
                        .insert(send_id, index);
                }
            }
        }


        // Check the same for FIND
        if let Some(find_actions) = self.rule.find.as_mut() {
            let count = find_actions.len();
            self.find_ref = Some(std::collections::HashMap::default());
            for (index, find_action) in find_actions.iter_mut().enumerate() {
                if let Err(err) = find_action.check_up(self.send_ref.as_ref(), count) {
                    return Err(self.make_error(Some(err)));
                }

                if let Some(find_id) = find_action.get_id() {
                    self.find_ref
                        .as_mut()
                        .unwrap()
                        .insert(find_id, index);
                }
            }
        }

        
        // Check the same for GET
        if let Some(get_actions) = self.rule.get.as_mut() {
            for get_action in get_actions.iter_mut() {
                if let Err(err) = get_action.check_up(self.find_ref.as_ref()) {
                    return Err(self.make_error(Some(err)));
                }
            }
        }

        Ok(())
    }

    pub(crate) fn get_change_placement_by_index(&self, index: usize) -> Result<ChangeValuePlacement, AuditError> {
        if let Some(change_list) = self.rule.change.as_ref() {
            let Some(change) = change_list.get(index) else {
                let err_str = format!("Tried to access change action with index {} in rule '{}', but it contains only {} change actions", index, self.id.as_str(), change_list.len());
                return Err(AuditError(err_str));
            };

            return Ok(change.placement_cache.as_ref().unwrap().clone());
        }
        else {
            let err_str = format!("Rule '{}' does not contain any change action, but try to access it occured", self.id.as_str());
            return Err(AuditError(err_str));
        }
    }

    pub(crate) fn get_payloads_by_index(&self, index: usize) -> Result<&Vec<String>, AuditError> {
        if let Some(change_list) = self.rule.change.as_ref() {
            let Some(change) = change_list.get(index) else {
                let err_str = format!("Tried to access change action with index {} in rule '{}', but it contains only {} change actions", index, self.id.as_str(), change_list.len());
                return Err(AuditError(err_str));
            };

            return Ok(&change.values);
        }
        else {
            let err_str = format!("Rule '{}' does not contain any change action, but try to access it occured", self.id.as_str());
            return Err(AuditError(err_str));
        }
    }

    pub fn get_send_actions_number(&self) -> usize {
        if let Some(send_actions) = self.rule.send.as_ref() {
            return send_actions.len();
        }
        else {
            return 0;
        }
    }

    pub fn get_watch_actions_number(&self) -> usize {
        if let Some(actions) = self.rule.watch.as_ref() {
            actions.len()
        }
        else {
            0
        }
    }
    
    pub(crate) fn get_id(&self) -> &str {
        return &self.id;
    }

    pub(crate) fn watch_actions(&self) -> Option<&Vec<RuleWatchAction>> {
        if let Some(watch_actions) = self.rule.watch.as_ref() {
            Some(watch_actions)
        }
        else {
            None
        }
    }
}
