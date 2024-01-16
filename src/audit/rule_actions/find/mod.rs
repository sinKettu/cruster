mod expression_args;
mod expression_methods;

use std::{collections::{HashMap, HashSet}, str::FromStr};

use serde::{Deserialize, Serialize};

use self::{expression_args::ExecutableExpressionArgsTypes, expression_methods::ExecutableExpressionMethod};

use super::{*, send::SendActionResults};


#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub(crate) enum LookFor {
    ANY,
    ALL
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub(crate) struct ExecutableExpression {
    name: String,
    operation_name: String,
    args: Vec<expression_args::ExecutableExpressionArg>,

    operation_cache: Option<ExecutableExpressionMethod>
}


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

        let mut existing_operations: HashMap<&str, ExecutableExpressionArgsTypes> = HashMap::with_capacity(self.exec.len());
        for operation in self.exec.iter_mut() {
            let operation_name = operation.operation_name.to_lowercase().replace("_", "");
            let method = match operation_name.as_str() {
                "len" => {
                    expression_methods::ExecutableExpressionMethod::LEN
                },

                "equal" => {
                    expression_methods::ExecutableExpressionMethod::EQUAL
                },
                "=" => {
                    expression_methods::ExecutableExpressionMethod::EQUAL
                },

                "greater" => {
                    expression_methods::ExecutableExpressionMethod::GREATER
                },
                ">" => {
                    expression_methods::ExecutableExpressionMethod::GREATER
                },

                "greaterorequal" => {
                    expression_methods::ExecutableExpressionMethod::GreaterOrEqual
                },
                ">=" => {
                    expression_methods::ExecutableExpressionMethod::GreaterOrEqual
                },

                "less" => {
                    expression_methods::ExecutableExpressionMethod::LESS
                },
                "<" => {
                    expression_methods::ExecutableExpressionMethod::LESS
                },

                "lessorequal" => {
                    expression_methods::ExecutableExpressionMethod::LessOrEqual
                },
                "<=>" => {
                    expression_methods::ExecutableExpressionMethod::GreaterOrEqual
                },

                "rematch" => {
                    expression_methods::ExecutableExpressionMethod::ReMatch
                },
                "~" => {
                    expression_methods::ExecutableExpressionMethod::ReMatch
                },

                _ => {
                    let err_str = format!("Found unknown operation type - {} - at operation {}", &operation.operation_name, &operation.name);
                    return Err(AuditError(err_str));
                }
            };

            existing_operations.insert(&operation.name, method.get_type());

            for (index, arg) in operation.args.iter_mut().enumerate() {
                let arg_value = match arg.r#type.as_str() {
                    "string" => {
                        expression_args::ExecutableExpressionArgsValues::String(arg.value.clone())
                    },
                    "int" => {
                        let Ok(parsed_value) = arg.value.parse() else {
                            let err_str = format!("Could not value with index {} as i64 in '{}'", index, &operation.name);
                            return Err(AuditError(err_str));
                        };

                        expression_args::ExecutableExpressionArgsValues::Integer(parsed_value)
                    },
                    "bool" => {
                        let Ok(parsed_value) = arg.value.parse() else {
                            let err_str = format!("Could not value with index {} as bool in '{}'", index, &operation.name);
                            return Err(AuditError(err_str));
                        };

                        expression_args::ExecutableExpressionArgsValues::Boolean(parsed_value)
                    },
                    "reference" => {
                        let parts: Vec<&str> = arg.value.split(".").collect();
                        if parts.len() < 3 || parts.len() > 4 {
                            let err_str = format!("Could not parse value with index {} as reference to requests/response in '{}' because of wrong format", index, &operation.name);
                            return Err(AuditError(err_str));
                        }

                        let id = parts[0];

                        let pair_part = match parts[1] {
                            "request" => expression_args::PairPart::REQUEST,
                            "response" => expression_args::PairPart::RESPONSE,
                            _ => {
                                let err_str = format!("Could not parse value with index {} as reference to requests/response in '{}' because of unknown pair part: '{}'", index, &operation.name, parts[1]);
                                return Err(AuditError(err_str));
                            }
                        };

                        let message_part = if parts.len() == 3 {
                            match parts[2] {
                                "method" => {
                                    if let expression_args::PairPart::RESPONSE = pair_part {
                                        let err_str = format!("Could not parse value with index {} as reference to requests/response in '{}', trying to access method in response", index, &operation.name);
                                        return Err(AuditError(err_str));
                                    }

                                    expression_args::MessagePart::METHOD
                                },
                                "path" => {
                                    if let expression_args::PairPart::RESPONSE = pair_part {
                                        let err_str = format!("Could not parse value with index {} as reference to requests/response in '{}', trying to access path in response", index, &operation.name);
                                        return Err(AuditError(err_str));
                                    }

                                    expression_args::MessagePart::PATH
                                },
                                "version" => {
                                    expression_args::MessagePart::VERSION
                                },
                                "body" => {
                                    expression_args::MessagePart::BODY
                                },
                                "status" => {
                                    if let expression_args::PairPart::REQUEST = pair_part {
                                        let err_str = format!("Could not parse value with index {} as reference to requests/response in '{}', trying to access status in request", index, &operation.name);
                                        return Err(AuditError(err_str));
                                    }

                                    expression_args::MessagePart::STATUS
                                },
                                "headers" => {
                                    let err_str = format!("Could not parse value with index {} as reference to requests/response in '{}', trying to access header without name", index, &operation.name);
                                    return Err(AuditError(err_str));
                                },
                                _ => {
                                    let err_str = format!("Could not parse value with index {} as reference to requests/response in '{}', trying to access unknown message part: {}", index, &operation.name, parts[2]);
                                    return Err(AuditError(err_str));
                                }
                            }
                        }
                        else {
                            let header_name = parts[3];
                            if parts[2] == "headers" {
                                expression_args::MessagePart::HEADER(header_name.to_string())
                            }
                            else {
                                let err_str = format!("Could not parse value with index {} as reference to requests/response in '{}', incorrect/unknwon message part: {}", index, &operation.name, parts[2]);
                                return Err(AuditError(err_str));
                            }
                        };

                        expression_args::ExecutableExpressionArgsValues::Reference(
                            expression_args::Reference {
                                id: id.to_string(),
                                pair_part,
                                message_part
                            }
                        )
                    },
                    "variable" => {
                        let Some(op_type) = existing_operations.get(arg.value.as_str()) else {
                            let err_str = format!("Trying to access result of operation '{}' as variable from '{}', but no such operations exists (operation must be defined before usage)", &arg.value, &operation.name);
                            return Err(AuditError(err_str));
                        };

                        expression_args::ExecutableExpressionArgsValues::Variable((arg.value.clone(), op_type.clone()))
                    },
                    _ => {
                        let err_str = format!("Found argument with unknown type - {} - in operation {}", &arg.r#type, &operation.name);
                        return Err(AuditError(err_str));
                    }
                };

                arg.type_cache = Some(arg_value);
            }

            if let Err(err) = method.check_args(&operation.args) {
                let err_str = format!("Error in operation '{}': {}", &operation.name, err);
                return Err(AuditError(err_str));
            }
        }

        
        Ok(())
    }

    pub(crate) fn get_id(&self) -> Option<String> {
        self.id.clone()
    }

    pub(crate) fn exec(&self, send_actions_ref: &std::collections::HashMap<String, usize>, send_results: &Vec<SendActionResults>) -> Result<bool, AuditError> {
        todo!()
    }
}

