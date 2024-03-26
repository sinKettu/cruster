pub(crate) mod active;

use std::{collections::HashMap, fmt::Display};

use self::active::ActiveRule;

use super::{actions::{ChangeValuePlacement, RuleFindAction, RuleWatchAction}, AuditError, Rule, RuleByProtocal, RuleType};


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
        let actions = self.http_active_rule_mut()?;

        // Check variable values in Watch struct and fill .watch_ref
        let mut watch_ref = HashMap::with_capacity(actions.watch.len());
        for (index, watch_action) in actions.watch.iter_mut().enumerate() {
            if let Err(err) = watch_action.check_up() {
                return Err(self.make_error(Some(err)));
            }
            
            if let Some(watch_id) = watch_action.get_id() {
                watch_ref.insert(watch_id, index);
            }
        }
        actions.watch_ref = Some(watch_ref);
        

        // Check variable values and references in Change struct and fill .change_ref
        let mut change_ref = HashMap::with_capacity(actions.change.len());
        for (index, change_action) in actions.change.iter_mut().enumerate() {
            if let Err(err) = change_action.check_up(actions.watch_ref.as_ref().unwrap()) {
                return Err(self.make_error(Some(err)));
            }

            if let Some(change_id) = change_action.get_id() {
                change_ref.insert(change_id, index);
            }
        }
        actions.change_ref = Some(change_ref);


        // Check variable values and references in SEND struct and fill .send_ref
        let mut send_ref = HashMap::with_capacity(actions.send.len());
        for (index, send_action) in actions.send.iter_mut().enumerate() {
            if let Err(err) = send_action.check_up(actions.change_ref.as_ref()) {
                return Err(self.make_error(Some(err)));
            }

            if let Some(send_id) = send_action.get_id() {
                send_ref.insert(send_id, index);
            }
        }
        actions.send_ref = Some(send_ref);


        // Check the same for FIND
        let mut find_ref = HashMap::with_capacity(actions.find.len());
        let count = actions.find.len();
        for (index, find_action) in actions.find.iter_mut().enumerate() {
            if let Err(err) = find_action.check_up(actions.send_ref.as_ref(), count) {
                return Err(self.make_error(Some(err)));
            }

            if let Some(find_id) = find_action.get_id() {
                find_ref.insert(find_id, index);
            }
        }
        actions.find_ref = Some(find_ref);
        
        // Check the same for GET
        if let Some(get_actions) = actions.get.as_mut() {
            for get_action in get_actions.iter_mut() {
                if let Err(err) = get_action.check_up(actions.find_ref.as_ref(), actions.send_ref.as_ref()) {
                    return Err(self.make_error(Some(err)));
                }
            }
        }

        Ok(())
    }

    pub(crate) fn http_active_rule(&self) -> Result<&ActiveRule, AuditError> {
        match &self.rule {
            RuleByProtocal::Http(rule_type) => {
                match rule_type {
                    RuleType::Active(rule) => {
                        Ok(rule)
                    },
                    RuleType::Passive => {
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
                    RuleType::Passive => {
                        let err_str = format!("trying to work with '{}' like active rule, but it's passive", id);
                        Err(AuditError(err_str))
                    }
                }
            }
        }
    }

    pub(crate) fn get_change_placement_by_index(&self, index: usize) -> Result<ChangeValuePlacement, AuditError> {
        let rule = self.http_active_rule()?;
        let Some(change) = rule.change.get(index) else {
            let err_str = format!("Tried to access change action with index {} in rule '{}', but it contains only {} change actions", index, self.id.as_str(), rule.change.len());
            return Err(AuditError(err_str));
        };

        return Ok(change.placement_cache.as_ref().unwrap().clone());
    }

    pub(crate) fn get_payloads_by_index(&self, index: usize) -> Result<&Vec<String>, AuditError> {
        let rule = self.http_active_rule()?;
        let Some(change) = rule.change.get(index) else {
            let err_str = format!("Tried to access change action with index {} in rule '{}', but it contains only {} change actions", index, self.id.as_str(), rule.change.len());
            return Err(AuditError(err_str));
        };

        return Ok(&change.values);
    }

    pub fn get_send_actions_number(&self) -> Result<usize, AuditError> {
        let rule = self.http_active_rule()?;
        Ok(rule.send.len())
    }

    pub fn get_watch_actions_number(&self) -> Result<usize, AuditError> {
        let rule = self.http_active_rule()?;
        Ok(rule.watch.len())
    }
    
    pub(crate) fn get_id(&self) -> &str {
        return &self.id;
    }

    pub(crate) fn watch_actions(&self) -> Result<&Vec<RuleWatchAction>, AuditError> {
        let rule = self.http_active_rule()?;
        Ok(&rule.watch)
    }

    pub(crate) fn get_find_actions(&self) -> Result<&Vec<RuleFindAction>, AuditError> {
        let rule = self.http_active_rule()?;
        Ok(&rule.find)
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
