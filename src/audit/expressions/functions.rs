use std::str::FromStr;

use serde::{Serialize, Deserialize};

use crate::audit::AuditError;

use super::traits::*;
use super::args::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub(crate) enum GenericArg {
    Arg(FunctionArg),
    Function(Function)
}

impl KnownType for GenericArg {
    fn is_boolean(&self) -> bool {
        match self {
            GenericArg::Arg(a) => a.is_boolean(),
            GenericArg::Function(f) => f.is_boolean()
        }
    }

    fn is_integer(&self) -> bool {
        match self {
            GenericArg::Arg(a) => a.is_integer(),
            GenericArg::Function(f) => f.is_integer()
        }
    }

    fn is_string(&self) -> bool {
        match self {
            GenericArg::Arg(a) => a.is_string(),
            GenericArg::Function(f) => f.is_string()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub(crate) enum FunctionType {
    CompareInteger(CompareIntegerFunction),
    StringLength,
    UNDEFINED
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub(crate) enum CompareIntegerFunction {
    Equal,
    Greater,
    GreaterOrEqual,
    Lower,
    LowerOrEqual
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub(crate) struct Function {
    function: FunctionType,
    args: Vec<GenericArg>
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
enum FunctionReturnTypes {
    BOOLEAN,
    STRING,
    INTEGER,
    NULL
}

impl Default for Function {
    fn default() -> Self {
        Function { function: FunctionType::UNDEFINED, args: Vec::with_capacity(3) }
    }
}

impl Function {
    fn get_type(&self) -> FunctionReturnTypes {
        match &self.function {
            FunctionType::CompareInteger(_) => {
                return FunctionReturnTypes::BOOLEAN
            },
            FunctionType::StringLength => {
                return FunctionReturnTypes::INTEGER
            }
            _ => {
                return FunctionReturnTypes::NULL
            }
        }
    }

    pub(crate) fn set_function(&mut self, generic_function: FunctionType) -> Result<(), AuditError> {
        match &self.function {
            FunctionType::UNDEFINED => {},
            _ => {
                return Err(AuditError::from_str("Check your expression, probably it has two or more equal-priority operations/functions and it causes the error.").unwrap());
            }
        }

        match &generic_function {
            FunctionType::CompareInteger(_) => {
                if self.args.len() == 0 {
                    self.function = generic_function;
                    return Ok(())
                }
                else if self.args.len() <= 2 {
                    if self.args.iter_mut().all(|arg| { arg.is_integer() }) {
                        self.function = generic_function;
                        return Ok(())
                    }
                    else {
                        return Err(AuditError::from_str("Function that compares integers has one or more non-integer arguments.").unwrap());
                    }
                }
                else {
                    return Err(AuditError::from_str("Function that compares integers has 3 or more arguments, must be 2.").unwrap());
                }
            },
            FunctionType::StringLength => {
                if self.args.len() > 1 {
                    return Err(AuditError::from_str("Function that calculates length of a string must have exactly 1 argument.").unwrap());
                }
                else if self.args.len() == 1 {
                    if self.args[0].is_string() {
                        self.function = generic_function;
                        return Ok(());
                    }
                    else {
                        return Err(AuditError::from_str("Function that calculates length of a string must have exactly 1 string argument.").unwrap());
                    }
                }
                else {
                    self.function = generic_function;
                    return Ok(());
                }
            }
            _ => {
                unreachable!("You have tried to explicitly assign UNDEFINED function and I do not know how you could. Please, contact me and tell it.")
            }
        }
    }

    pub(crate) fn add_arg(&mut self, arg: GenericArg) -> Result<(), AuditError> {
        match &self.function {
            FunctionType::CompareInteger(_) => {
                if self.args.len() >= 2 {
                    return Err(AuditError::from_str("Function that compares integers must take exactly 2 arguments.").unwrap());
                }
                else if ! arg.is_integer() {
                    let str_err = format!("Cannot assign non-integer argument to function that compares integers: '{:?}'", arg);
                    return Err(AuditError(str_err));
                }
                else {
                    self.args.push(arg);
                }
            },
            FunctionType::StringLength => {
                if self.args.len() >= 1 {
                    return Err(AuditError::from_str("Function returning length of a string must take exactly 1 argument.").unwrap());
                }
                else {
                    if ! arg.is_string() {
                        let str_err = format!("Cannot assign non-string argument to function that calculates length of a string: '{:?}'", arg);
                        return Err(AuditError(str_err));
                    }
                    else {
                        self.args.push(arg);
                    }
                }
            },
            FunctionType::UNDEFINED => {
                self.args.push(arg);
            },
        }

        Ok(())
    }
}

impl KnownType for Function {
    fn is_boolean(&self) -> bool {
        return self.get_type() == FunctionReturnTypes::BOOLEAN;
    }

    fn is_integer(&self) -> bool {
        return self.get_type() == FunctionReturnTypes::INTEGER;
    }

    fn is_string(&self) -> bool {
        return self.get_type() == FunctionReturnTypes::STRING;
    }
}

impl ExecutableFunction for Function {
    fn execute(&mut self) -> Result<FunctionArg, AuditError> {
        let func_ref = &mut self.function;

        let mut args: Vec<FunctionArg> = Vec::with_capacity(self.args.len());
        for fut_arg in self.args.iter_mut() {
            match fut_arg {
                GenericArg::Arg(arg) => { args.push(arg.clone()) },
                GenericArg::Function(func) => { args.push(func.execute()?) }
            }
        }

        match func_ref {
            FunctionType::CompareInteger(func) => {
                let result = match func {
                    CompareIntegerFunction::Equal => {
                        args[0].integer().unwrap() == args[1].integer().unwrap()
                    },
                    CompareIntegerFunction::Greater => {
                        args[0].integer().unwrap() > args[1].integer().unwrap()
                    },
                    CompareIntegerFunction::GreaterOrEqual => {
                        args[0].integer().unwrap() >= args[1].integer().unwrap()
                    },
                    CompareIntegerFunction::Lower => {
                        args[0].integer().unwrap() < args[1].integer().unwrap()
                    },
                    CompareIntegerFunction::LowerOrEqual => {
                        args[0].integer().unwrap() <= args[1].integer().unwrap()
                    }
                };

                return Ok(FunctionArg::BOOLEAN(result));
            },
            FunctionType::StringLength => {
                return Ok(
                    FunctionArg::INTEGER(
                        args[0]
                            .string()
                            .unwrap()
                            .len()
                    )
                )
            }
            FunctionType::UNDEFINED => {
                return Err(AuditError::from_str("Cannot execute an undefined function. Check expression.").unwrap())
            }
        }
    }
}