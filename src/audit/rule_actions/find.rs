use std::{collections::HashMap, str::FromStr};

use super::*;

impl RuleFindAction {
    pub(crate) fn check_up(&mut self, _possible_send_ref: Option<&HashMap<String, usize>>) -> Result<(), AuditError> {
        let lowercase_look_for = self.look_for.to_lowercase();
        match lowercase_look_for.as_str() {
            "any" => {
                self.look_for_cache = Some(LookFor::ANY);
            },
            "all" => {
                self.look_for_cache = Some(LookFor::ALL);
            },
            _ => {
                return Err(
                    AuditError::from_str(
                        format!("unsupported look_for statement: {}", &self.look_for).as_str()
                    ).unwrap()
                );
            }
        }

        // TODO: validate and eval expressions in .expressions

        Ok(())
    }

    pub(crate) fn get_id(&self) -> Option<String> {
        self.id.clone()
    }
}
