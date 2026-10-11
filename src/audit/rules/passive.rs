use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::audit::{actions::{RuleFindAction, RuleGetAction}, AuditError};


#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub(crate) struct PassiveRule {
    pub(crate) find: Vec<RuleFindAction>,
    pub(crate) get: Option<Vec<RuleGetAction>>,

    // These are "service" fields, to be used by cruster
    pub(crate) find_ref: Option<HashMap<String, usize>>,
}

impl PassiveRule {
    pub(super) fn check_up(&mut self) -> Result<(), AuditError> {
        // to refactor less
        let actions = self;

        // Check variable values in FIND structure and fill .watch_ref
        let mut find_ref = HashMap::with_capacity(actions.find.len());
        let count = actions.find.len();
        for (index, find_action) in actions.find.iter_mut().enumerate() {
            find_action.check_up(None, count)?;
            if let Some(find_id) = find_action.get_id() {
                find_ref.insert(find_id, index);
            }
        }
        actions.find_ref = Some(find_ref);
        
        // Check the same for GET
        if let Some(get_actions) = actions.get.as_mut() {
            for get_action in get_actions.iter_mut() {
                get_action.check_up(actions.find_ref.as_ref(), None)?;
            }
        }

        Ok(())
    }
}