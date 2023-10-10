use std::{collections::HashMap, str::FromStr};

use super::*;
use crate::audit::expressions;

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

        let mut parsed_expressions: Vec<Function> = Vec::with_capacity(self.expressions.len());
        for (index, str_exp) in self.expressions.iter().enumerate() {
            match expressions::parse_expression(str_exp) {
                Ok(expr) => parsed_expressions.push(expr),
                Err(err) => {
                    let err_str = format!("Expression {} has an error: {}", index, err);
                    return Err(AuditError(err_str));
                }
            }
        }

        self.parsed_expressions = Some(parsed_expressions);

        Ok(())
    }

    pub(crate) fn get_id(&self) -> Option<String> {
        self.id.clone()
    }
}

