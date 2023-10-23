use std::str::FromStr;

use serde::{Serialize, Deserialize};
use regex;

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

    fn get_type(&self) -> ArgType {
        match &self {
            GenericArg::Arg(a) => a.get_type(),
            GenericArg::Function(f) => f.get_type()
        }
    }
}

impl GenericArg {
    // The method requires manual check when new type is added
    fn has_the_same_type_as(&self, arg: &ArgType) -> bool {
        return self.get_type() == *arg;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub(crate) enum FunctionType {
    CompareInteger(CompareIntegerFunction),
    StringLength,
    MatchString,
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

impl Default for Function {
    fn default() -> Self {
        Function { function: FunctionType::UNDEFINED, args: Vec::with_capacity(3) }
    }
}

impl Function {
    fn check_args(&self, args_number_cond: usize, arg1_type_cond: ArgType, arg2_type_cond: Option<ArgType>) -> Result<(), AuditError> {
        if self.args.len() > args_number_cond {
            return Err(AuditError(format!("Function has {} arguments, but takes only {}.", self.args.len(), args_number_cond)));
        }

        if self.args.len() >= 1 {
            if ! self.args[0].has_the_same_type_as(&arg1_type_cond) {
                let err = format!("Function has the first argument with type {:?}, but {:?} is expected.", self.args[0].get_type(), arg1_type_cond);
                return Err(AuditError(err));
            }
        }

        if let Some(arg2tc) = arg2_type_cond.as_ref() {
            if self.args.len() >= 2 {
                if ! self.args[1].has_the_same_type_as(&arg2tc) {
                    let err = format!("Function has the second argument with type {:?}, but {:?} is expected.", self.args[1].get_type(), arg2tc);
                    return Err(AuditError(err));
                }
            }
        }

        return Ok(())
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
                if let Err(err) =  self.check_args(2, ArgType::INTEGER, Some(ArgType::INTEGER)) {
                    return Err(AuditError(format!("Error in integer comparison: {}", err)));
                }
                else {
                    self.function = generic_function;
                    return Ok(());
                }
            },
            FunctionType::StringLength => {
                if let Err(err) = self.check_args(1, ArgType::STRING, None) {
                    return Err(AuditError(format!("Error in string length retrieving: {}", err)));
                }
                else {
                    self.function = generic_function;
                    return Ok(());
                }
            },
            FunctionType::MatchString => {
                if let Err(err) = self.check_args(2, ArgType::STRING, Some(ArgType::STRING)) {
                    return Err(AuditError(format!("Error in string match: {}", err)));
                }
                else {
                    self.function = generic_function;
                    return Ok(());
                }
            }
            FunctionType::UNDEFINED => {
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
            FunctionType::MatchString => {
                if self.args.len() >= 2 {
                    return Err(AuditError::from_str("Function matching string takes exactly 2 arguments, but 3 or more were given").unwrap());
                }
                else {
                    if ! arg.is_string() {
                        let str_err = format!("Cannot assign non-string argument to function that matches string: '{:?}'", arg);
                        return Err(AuditError(str_err));
                    }
                    else {
                        self.args.push(arg);
                    }
                }
            }
            FunctionType::UNDEFINED => {
                self.args.push(arg);
            },
        }

        Ok(())
    }
}

impl KnownType for Function {
    fn get_type(&self) -> ArgType {
        match &self.function {
            FunctionType::CompareInteger(_) => {
                return ArgType::BOOLEAN
            },
            FunctionType::StringLength => {
                return ArgType::INTEGER
            },
            FunctionType::MatchString => {
                return ArgType::BOOLEAN
            }
            FunctionType::UNDEFINED => {
                return ArgType::NULL
            }
        }
    }

    fn is_boolean(&self) -> bool {
        return self.get_type() == ArgType::BOOLEAN;
    }

    fn is_integer(&self) -> bool {
        return self.get_type() == ArgType::INTEGER;
    }

    fn is_string(&self) -> bool {
        return self.get_type() == ArgType::STRING;
    }
}

impl ExecutableFunction for Function {
    fn execute(&self) -> Result<FunctionArg, AuditError> {
        let func_ref = &self.function;

        let mut args: Vec<FunctionArg> = Vec::with_capacity(self.args.len());
        for fut_arg in self.args.iter() {
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
            },
            FunctionType::MatchString => {
                let str_re = args[0].string().unwrap();
                let re = match regex::Regex::new(&str_re) {
                    Ok(re) => re,
                    Err(err) => return Err(AuditError(format!("Could not parse regex in string match function: {}", err)))
                };

                let str_arg = args[1].string().unwrap();
                let result = re.is_match(&str_arg);

                return Ok(
                    FunctionArg::BOOLEAN(
                        result
                    )
                );
            }
            FunctionType::UNDEFINED => {
                return Err(AuditError::from_str("Cannot execute an undefined function. Check expression.").unwrap())
            }
        }
    }
}