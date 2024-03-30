use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::audit::{actions::{RuleChangeAction, RuleFindAction, RuleGetAction, RuleSendAction, RuleWatchAction}, AuditError};


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

impl ActiveRule {
    pub(super) fn check_up(&mut self) -> Result<(), AuditError> {
        // to refactor less
        let actions = self;

        // Check variable values in Watch struct and fill .watch_ref
        let mut watch_ref = HashMap::with_capacity(actions.watch.len());
        for (index, watch_action) in actions.watch.iter_mut().enumerate() {
            watch_action.check_up()?;
            if let Some(watch_id) = watch_action.get_id() {
                watch_ref.insert(watch_id, index);
            }
        }
        actions.watch_ref = Some(watch_ref);
        

        // Check variable values and references in Change struct and fill .change_ref
        let mut change_ref = HashMap::with_capacity(actions.change.len());
        for (index, change_action) in actions.change.iter_mut().enumerate() {
            change_action.check_up(actions.watch_ref.as_ref().unwrap())?;
            if let Some(change_id) = change_action.get_id() {
                change_ref.insert(change_id, index);
            }
        }
        actions.change_ref = Some(change_ref);


        // Check variable values and references in SEND struct and fill .send_ref
        let mut send_ref = HashMap::with_capacity(actions.send.len());
        for (index, send_action) in actions.send.iter_mut().enumerate() {
            send_action.check_up(actions.change_ref.as_ref())?;
            if let Some(send_id) = send_action.get_id() {
                send_ref.insert(send_id, index);
            }
        }
        actions.send_ref = Some(send_ref);


        // Check the same for FIND
        let mut find_ref = HashMap::with_capacity(actions.find.len());
        let count = actions.find.len();
        for (index, find_action) in actions.find.iter_mut().enumerate() {
            find_action.check_up(actions.send_ref.as_ref(), count)?;
            if let Some(find_id) = find_action.get_id() {
                find_ref.insert(find_id, index);
            }
        }
        actions.find_ref = Some(find_ref);
        
        // Check the same for GET
        if let Some(get_actions) = actions.get.as_mut() {
            for get_action in get_actions.iter_mut() {
                get_action.check_up(actions.find_ref.as_ref(), actions.send_ref.as_ref())?;
            }
        }

        Ok(())
    }
}