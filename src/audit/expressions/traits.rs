use crate::audit::{AuditError, rule_actions::send::SendActionResults};

use super::args::{FunctionArg, ArgType};

pub(crate) trait ExecutableFunction: KnownType {
    fn execute(&self, send_id_ref: Option<&std::collections::HashMap<String, usize>>, send_results: Option<&Vec<SendActionResults>>) -> Result<FunctionArg, AuditError>;
}

pub(crate) trait IntoFunctionArg: KnownType {
    fn arg(&mut self) -> Result<FunctionArg, AuditError>;
    fn with_deref(&mut self, send_actions_ref: &std::collections::HashMap<String, usize>, send_results: &Vec<SendActionResults>) -> Result<FunctionArg, AuditError>;
}

pub(crate) trait KnownType {
    fn is_string(&self) -> bool;
    fn is_integer(&self) -> bool;
    fn is_boolean(&self) -> bool;
    fn get_type(&self) -> ArgType;
}
