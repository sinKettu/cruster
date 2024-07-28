pub(crate) mod args;
mod methods;
mod operations;

use std::collections::HashMap;

use log::debug;
use serde::{Deserialize, Serialize};

use crate::audit::contexts::traits::{WithFindAction, WithSendAction};

use self::{args::{ExecutableExpressionArgsTypes, ExecutableExpressionArgsValues}, methods::ExecutableExpressionMethod};

use super::*;


#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub(crate) enum LookFor {
    ANY,
    ALL
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub(crate) struct ExecutableExpression {
    name: String,
    operation: String,
    args: Vec<args::ExecutableExpressionArg>,

    operation_cache: Option<ExecutableExpressionMethod>
}

impl ExecutableExpression {
    fn exec(&self, args: &Vec<ExecutableExpressionArgsValues>) -> Result<ExecutableExpressionArgsValues, AuditError> {
        self.operation_cache.as_ref().unwrap().exec(args)
    }
}


impl RuleFindAction {
    pub(crate) fn get_required_send_results(&self) -> &HashSet<usize> {
        self.required_send_actions.as_ref().unwrap()
    }

    pub(crate) fn check_up(&mut self, possible_send_ref: Option<&HashMap<String, usize>>, send_actions_count: usize) -> Result<(), AuditError> {
        // let lowercase_look_for = self.look_for.to_lowercase();
        // match lowercase_look_for.as_str() {
        //     "any" => {
        //         self.look_for_cache = Some(LookFor::ANY);
        //     },
        //     "all" => {
        //         self.look_for_cache = Some(LookFor::ALL);
        //     },
        //     _ => {
        //         return Err(
        //             AuditError::from_str(
        //                 format!("unsupported look_for statement: {}", &self.look_for).as_str()
        //             ).unwrap()
        //         );
        //     }
        // }

        let mut existing_operations: HashMap<&str, ExecutableExpressionArgsTypes> = HashMap::with_capacity(self.exec.len());
        let mut required_send_actions: HashSet<usize> = HashSet::with_capacity(4);

        for operation in self.exec.iter_mut() {
            let operation_name = operation.operation.to_lowercase().replace("_", "");
            let method = match operation_name.as_str() {
                "len" => {
                    methods::ExecutableExpressionMethod::LEN
                },
                "equal" | "=" => {
                    methods::ExecutableExpressionMethod::EQUAL
                },
                "greater" | ">" => {
                    methods::ExecutableExpressionMethod::GREATER
                },
                "greaterorequal" | ">=" => {
                    methods::ExecutableExpressionMethod::GreaterOrEqual
                },
                "less" | "<" => {
                    methods::ExecutableExpressionMethod::LESS
                },
                "lessorequal" | "<=" => {
                    methods::ExecutableExpressionMethod::LessOrEqual
                },
                "rematch" | "~" => {
                    methods::ExecutableExpressionMethod::ReMatch
                },
                "and" | "&&" => {
                    methods::ExecutableExpressionMethod::AND
                },
                "or" | "||" => {
                    methods::ExecutableExpressionMethod::OR
                },
                _ => {
                    let err_str = format!("Found unknown operation type - {} - at operation {}", &operation.operation, &operation.name);
                    return Err(AuditError(err_str));
                }
            };

            existing_operations.insert(&operation.name, method.get_type());

            for (index, arg) in operation.args.iter_mut().enumerate() {
                let arg_value = match arg.r#type.as_str() {
                    "string" => {
                        args::ExecutableExpressionArgsValues::String(arg.value.clone())
                    },
                    "int" => {
                        let Ok(parsed_value) = arg.value.parse() else {
                            let err_str = format!("Could not value with index {} as i64 in '{}'", index, &operation.name);
                            return Err(AuditError(err_str));
                        };

                        args::ExecutableExpressionArgsValues::Integer(parsed_value)
                    },
                    "bool" => {
                        let Ok(parsed_value) = arg.value.parse() else {
                            let err_str = format!("Could not value with index {} as bool in '{}'", index, &operation.name);
                            return Err(AuditError(err_str));
                        };

                        args::ExecutableExpressionArgsValues::Boolean(parsed_value)
                    },
                    "reference" => {
                        let parts: Vec<&str> = arg.value.split(".").collect();
                        if parts.len() < 3 || parts.len() > 4 {
                            let err_str = format!("Could not parse value with index {} as reference to requests/response in '{}' because of wrong format", index, &operation.name);
                            return Err(AuditError(err_str));
                        }

                        let id = parts[0];
                        let int_id = match id.parse::<usize>() {
                            Ok(i) => {
                                i + 1
                            },
                            Err(_) => {
                                if id == "initial" {
                                    0
                                }
                                else {
                                    match possible_send_ref {
                                        Some(send_ref) => {
                                            match send_ref.get(id) {
                                                Some(index) => {
                                                    index.to_owned() + 1
                                                },
                                                None => {
                                                    let err_str = format!("Could not parse Refrence ({} arg) in {}: could not resolve str id - {}", index, &operation.name, id);
                                                    return Err(AuditError(err_str));
                                                }
                                            }
                                        },
                                        None => {
                                            let err_str = format!("Could not parse Refrence ({} arg) in {}: could not resolve str id - {}, no mappings for resolving", index, &operation.name, id);
                                            return Err(AuditError(err_str));
                                        }
                                    }
                                }
                            }
                        };

                        if int_id > send_actions_count {
                            let err_str = format!("Refrence ({} arg) in {} resolved to index {}, but there are only {} send actions", index, &operation.name, int_id, send_actions_count);
                            return Err(AuditError(err_str));
                        }

                        let _ = required_send_actions.insert(int_id);

                        let pair_part = match parts[1] {
                            "request" => args::PairPart::REQUEST,
                            "response" => args::PairPart::RESPONSE,
                            _ => {
                                let err_str = format!("Could not parse value with index {} as reference to requests/response in '{}' because of unknown pair part: '{}'", index, &operation.name, parts[1]);
                                return Err(AuditError(err_str));
                            }
                        };

                        let message_part = if parts.len() == 3 {
                            match parts[2] {
                                "method" => {
                                    if let args::PairPart::RESPONSE = pair_part {
                                        let err_str = format!("Could not parse value with index {} as reference to requests/response in '{}', trying to access method in response", index, &operation.name);
                                        return Err(AuditError(err_str));
                                    }

                                    args::MessagePart::METHOD
                                },
                                "path" => {
                                    if let args::PairPart::RESPONSE = pair_part {
                                        let err_str = format!("Could not parse value with index {} as reference to requests/response in '{}', trying to access path in response", index, &operation.name);
                                        return Err(AuditError(err_str));
                                    }

                                    args::MessagePart::PATH
                                },
                                "version" => {
                                    args::MessagePart::VERSION
                                },
                                "body" => {
                                    args::MessagePart::BODY
                                },
                                "status" => {
                                    if let args::PairPart::REQUEST = pair_part {
                                        let err_str = format!("Could not parse value with index {} as reference to requests/response in '{}', trying to access status in request", index, &operation.name);
                                        return Err(AuditError(err_str));
                                    }

                                    args::MessagePart::STATUS
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
                                args::MessagePart::HEADER(header_name.to_string())
                            }
                            else {
                                let err_str = format!("Could not parse value with index {} as reference to requests/response in '{}', incorrect/unknwon message part: {}", index, &operation.name, parts[2]);
                                return Err(AuditError(err_str));
                            }
                        };

                        args::ExecutableExpressionArgsValues::Reference(
                            args::Reference {
                                id: int_id,
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

                        args::ExecutableExpressionArgsValues::Variable((arg.value.clone(), op_type.clone()))
                    },
                    _ => {
                        let err_str = format!("Found argument with unknown type - {} - in operation {}", &arg.r#type, &operation.name);
                        return Err(AuditError(err_str));
                    }
                };

                arg.type_cache = Some(arg_value);
            }

            if let Err(err) = method.check_args(&operation.args) {
                let err_str = format!("Error in operation '{}': {}", &operation.name, err.to_string());
                return Err(AuditError(err_str));
            }

            operation.operation_cache = Some(method);
        }

        self.required_send_actions = Some(required_send_actions);
        Ok(())
    }

    pub(crate) fn get_id(&self) -> Option<String> {
        self.id.clone()
    }

    pub(crate) fn exec<'pair_lt, 'rule_lt, T>(&self, ctxt: &mut T) -> Result<(), AuditError> 
    where
        T: WithFindAction<'pair_lt,> + WithSendAction<'pair_lt>
    {
        let mut executed: HashMap<&str, ExecutableExpressionArgsValues> = HashMap::with_capacity(self.exec.len());
        let mut last_op: &str = "";
        for op in self.exec.iter() {
            let mut args: Vec<ExecutableExpressionArgsValues> = Vec::with_capacity(op.args.len());
            for arg in op.args.iter() {
                match arg.type_cache.as_ref().unwrap() {
                    ExecutableExpressionArgsValues::Reference(refer) => {
                        let dereferenced = refer.deref(ctxt.initial_pair(), ctxt.send_results())?;
                        args.push(dereferenced);
                    },
                    ExecutableExpressionArgsValues::Variable((op_name, _)) => {
                        let Some(op_result) = executed.get(op_name.as_str()) else {
                            let err_str = format!("Cannot use result of operation '{}' as variable: operation does not exists or has not executed yet", op_name);
                            return Err(AuditError(err_str));
                        };

                        args.push(op_result.clone());
                    },
                    _ => {
                        args.push(arg.type_cache.clone().unwrap())
                    }
                }
            }

            let res = match op.exec(&args) {
                Ok(value) => {
                    value
                },
                Err(err) => {
                    let err_str = format!("Error on operation '{}': {}", op.name.as_str(), err.to_string());
                    return Err(AuditError(err_str))
                }
            };

            debug!("FindActions - executed operation '{} ({})', result: {:?}", &op.name, &op.operation, res); // think one more, it logs body

            last_op = op.name.as_str();
            let _ = executed.insert(last_op, res);
        }

        let last_result = &executed[last_op];
        
        if last_result.get_type() != ExecutableExpressionArgsTypes::BOOLEAN {
            let err_str = format!("Last operation ({}) in find action has type {:?}, but it should be BOOLEAN", last_op, last_result.get_type());
            return Err(AuditError(err_str));
        }

        match last_result {
            ExecutableExpressionArgsValues::Boolean(b) => {
                ctxt.add_find_result((b.to_owned(), None));
            },
            ExecutableExpressionArgsValues::Several(s) => {
                ctxt.add_find_result((s.iter().any(|i| { i.boolean() }), None));
            },
            ExecutableExpressionArgsValues::WithSendResReference(refer) => {
                match &refer.arg {
                    ExecutableExpressionArgsValues::Boolean(b) => {
                        ctxt.add_find_result((b.to_owned(), None));
                    },
                    ExecutableExpressionArgsValues::Several(s) => {
                        let mut found = false;
                        // It takes wrong index and i should understand why
                        for (index, val) in s.iter().enumerate() {
                            if val.boolean() {
                                // ctxt.add_find_result((true, Some(refer.refer[0].send_action_id)));
                                ctxt.add_find_result((true, Some(index)));
                                found = true;
                                break; // Instead of breaking we can find all occurancies - TODO
                            }
                        }

                        if ! found {
                            ctxt.add_find_result((false, None));
                        }
                    },
                    _ => {
                        let err_str = format!("Last operation ({}) in find action has type {:?} incapsulated in ExecutableExpressionArgsValues::WithSendResReference, but it should be BOOLEAN", last_op, last_result.get_type());
                        return Err(AuditError(err_str));
                    }
                }
            }
            _ => {
                let err_str = format!("Last operation ({}) in find action has type {:?}, but it should be BOOLEAN", last_op, last_result.get_type());
                return Err(AuditError(err_str));
            }
        }

        Ok(())
    }


}

