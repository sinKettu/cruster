use serde::{Serialize, Deserialize};

use crate::audit::AuditError;

use super::functions::GenericArg;
use super::traits::{IntoFunctionArg, KnownType};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub(crate) enum ArgType {
    STRING,
    INTEGER,
    BOOLEAN,
    NULL
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub(crate) enum FunctionArg {
    STRING(String),
    INTEGER(usize),
    BOOLEAN(bool),
    NULL
}

impl FunctionArg {
    // pub(crate) fn boxed(self) -> Box<Self> {
    //     Box::new(self)
    // }

    pub(crate) fn into_generic(self) -> GenericArg {
        GenericArg::Arg(self)
    }

    pub(crate) fn integer(&self) -> Option<usize> {
        match self {
            FunctionArg::INTEGER(i) => Some(i.clone()),
            _ => None
        }
    }

    pub(crate) fn string(&self) -> Option<String> {
        match self {
            FunctionArg::STRING(s) => Some(s.clone()),
            _ => None
        }
    }

    pub(crate) fn boolean(&self) -> Option<bool> {
        match self {
            FunctionArg::BOOLEAN(b) => Some(b.clone()),
            _ => None
        }
    }
}

impl IntoFunctionArg for FunctionArg {
    fn arg(&mut self) -> Result<FunctionArg, AuditError> {
        Ok(self.clone())
    }
}

impl KnownType for FunctionArg {
    fn is_boolean(&self) -> bool {
        return match self {
            FunctionArg::BOOLEAN(_) => { true },
            _ => { false }
        }
    }

    fn is_integer(&self) -> bool {
        return match self {
            FunctionArg::INTEGER(_) => { true },
            _ => { false }
        }
    }

    fn is_string(&self) -> bool {
        return match self {
            FunctionArg::STRING(_) => { true },
            _ => { false }
        }
    }

    fn get_type(&self) -> ArgType {
        return match &self {
            FunctionArg::STRING(_) => ArgType::STRING,
            FunctionArg::BOOLEAN(_) => ArgType::BOOLEAN,
            FunctionArg::INTEGER(_)  => ArgType::INTEGER,
            FunctionArg::NULL => ArgType::NULL
        }
    }
}
