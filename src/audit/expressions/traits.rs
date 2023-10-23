use crate::audit::AuditError;

use super::args::{FunctionArg, ArgType};

pub(crate) trait ExecutableFunction: KnownType {
    fn execute(&mut self) -> Result<FunctionArg, AuditError>;
}

pub(crate) trait IntoFunctionArg: KnownType {
    fn arg(&mut self) -> Result<FunctionArg, AuditError>;
}

pub(crate) trait KnownType {
    fn is_string(&self) -> bool;
    fn is_integer(&self) -> bool;
    fn is_boolean(&self) -> bool;
    fn get_type(&self) -> ArgType;
}
