use serde::{Deserialize, Serialize};

use crate::audit::{AuditError};

use super::{args::{ExecutableExpressionArg, ExecutableExpressionArgsTypes, ExecutableExpressionArgsValues}, ExecutableExpression};

use super::operations::Operations;

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub enum ExecutableExpressionMethod {
    LEN,

    GREATER,
    LESS,
    EQUAL,
    GreaterOrEqual,
    LessOrEqual,

    ReMatch
}

impl ExecutableExpressionMethod {
    pub(crate) fn get_type(&self) -> ExecutableExpressionArgsTypes {
        match self {
            ExecutableExpressionMethod::LEN => {
                ExecutableExpressionArgsTypes::INTEGER
            },
            ExecutableExpressionMethod::GREATER => {
                ExecutableExpressionArgsTypes::BOOLEAN
            },
            ExecutableExpressionMethod::LESS => {
                ExecutableExpressionArgsTypes::BOOLEAN
            },
            ExecutableExpressionMethod::EQUAL => {
                ExecutableExpressionArgsTypes::BOOLEAN
            },
            ExecutableExpressionMethod::GreaterOrEqual => {
                ExecutableExpressionArgsTypes::BOOLEAN
            },
            ExecutableExpressionMethod::LessOrEqual => {
                ExecutableExpressionArgsTypes::BOOLEAN
            },
            ExecutableExpressionMethod::ReMatch => {
                ExecutableExpressionArgsTypes::BOOLEAN
            },
        }
    }

    pub(crate) fn check_args(&self, args: &Vec<ExecutableExpressionArg>) -> Result<(), AuditError> {
        let args_have_the_same_type = | args: &Vec<ExecutableExpressionArg> | -> bool {
            let first_type: ExecutableExpressionArgsTypes = args[0].type_cache.as_ref().unwrap().get_type();
            for arg in args.iter().skip(1) {
                if first_type != arg.type_cache.as_ref().unwrap().get_type() {
                    return false;
                }
            }

            return true;
        };

        let args_have_exact_type = | args: &Vec<ExecutableExpressionArg>, expected_type: ExecutableExpressionArgsTypes | -> bool {
            for arg in args.iter() {
                if arg.type_cache.as_ref().unwrap().get_type() != expected_type {
                    return false;
                }
            }

            return true;
        };

        match self {
            ExecutableExpressionMethod::EQUAL => {
                if args.len() != 2 {
                    let err_str = format!("'Equal' operation expects 2 arguments, but {} were given", args.len());
                    return Err(AuditError(err_str));
                }

                if ! args_have_the_same_type(args) {
                    let err_str = "'Equal' operation expects all arguments have the same type".to_string();
                    return Err(AuditError(err_str));
                }
            },
            ExecutableExpressionMethod::GREATER => {
                if args.len() != 2 {
                    let err_str = format!("'Greater' operation expects 2 arguments, but {} were given", args.len());
                    return Err(AuditError(err_str));
                }

                if ! args_have_exact_type(args, ExecutableExpressionArgsTypes::INTEGER) {
                    let err_str = "'Greater' operation expects all arguments have the type 'integer'".to_string();
                    return Err(AuditError(err_str));
                }
            },
            ExecutableExpressionMethod::LESS => {
                if args.len() != 2 {
                    let err_str = format!("'Less' operation expects 2 arguments, but {} were given", args.len());
                    return Err(AuditError(err_str));
                }

                if ! args_have_exact_type(args, ExecutableExpressionArgsTypes::INTEGER) {
                    let err_str = "'Less' operation expects all arguments have the type 'integer'".to_string();
                    return Err(AuditError(err_str));
                }
            },
            ExecutableExpressionMethod::GreaterOrEqual => {
                if args.len() != 2 {
                    let err_str = format!("'GreaterOrEqual' operation expects 2 arguments, but {} were given", args.len());
                    return Err(AuditError(err_str));
                }

                if ! args_have_exact_type(args, ExecutableExpressionArgsTypes::INTEGER) {
                    let err_str = "'GreaterOrEqual' operation expects all arguments have the type 'integer'".to_string();
                    return Err(AuditError(err_str));
                }
            },
            ExecutableExpressionMethod::LessOrEqual => {
                if args.len() != 2 {
                    let err_str = format!("'LessOrEqual' operation expects 2 arguments, but {} were given", args.len());
                    return Err(AuditError(err_str));
                }

                if ! args_have_exact_type(args, ExecutableExpressionArgsTypes::INTEGER) {
                    let err_str = "'LessOrEqual' operation expects all arguments have the type 'integer'".to_string();
                    return Err(AuditError(err_str));
                }
            },
            ExecutableExpressionMethod::LEN => {
                if args.len() != 1 {
                    let err_str = format!("'Len' operation expects 1 argument, but {} were given", args.len());
                    return Err(AuditError(err_str));
                }

                if ! args_have_exact_type(args, ExecutableExpressionArgsTypes::STRING) {
                    let err_str = "'Len' operation expects argument of the type 'string'".to_string();
                    return Err(AuditError(err_str));
                }
            },
            ExecutableExpressionMethod::ReMatch => {
                if args.len() != 2 {
                    let err_str = format!("'ReMatch' operation expects 2 arguments, but {} were given", args.len());
                    return Err(AuditError(err_str));
                }

                if ! args_have_exact_type(args, ExecutableExpressionArgsTypes::STRING) {
                    let err_str = "'ReMatch' operation expects all arguments have the type 'string'".to_string();
                    return Err(AuditError(err_str));
                }
            },
        }

        Ok(())
    }

    pub(crate) fn exec(&self, args: &Vec<ExecutableExpressionArgsValues>) -> Result<ExecutableExpressionArgsValues, AuditError> {
        match self {
            Self::LEN => {
                Ok(args[0].len())
            },
            Self::ReMatch => {
                Ok(args[0].re_match(&args[1]))
            },
            Self::EQUAL => {
                Ok(args[0].equal(&args[1]))
            },
            Self::GREATER => {
                Ok(args[0].greater(&args[1]))
            },
            Self::LESS => {
                Ok(args[0].less(&args[1]))
            },
            Self::LessOrEqual => {
                Ok(args[0].less_or_equal(&args[1]))
            },
            Self::GreaterOrEqual => {
                Ok(args[0].greater_or_equal(&args[1]))
            },
        }   
    }
}


