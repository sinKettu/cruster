use std::borrow::Cow;

use bstr::ByteSlice;
use serde::{Serialize, Deserialize};

use crate::audit::AuditError;

use super::functions::GenericArg;
use super::traits::{IntoFunctionArg, KnownType};


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub(crate) enum PairPart {
    REQUEST,
    RESPONSE
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub(crate) enum MessagePart {
    METHOD,
    PATH,
    VERSION,
    STATUS,
    HEADER(String),
    BODY
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub(crate) struct Reference {
    pub(crate) id: String,
    pub(crate) pair_part: PairPart,
    pub(crate) message_part: MessagePart
}

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
    REF(Reference),
    SEVERAL(Vec<FunctionArg>),
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

    fn type_of_several(&self, array: &Vec<FunctionArg>) -> ArgType {
        if array.len() == 0 {
            return ArgType::NULL
        }
        else {
            return array[0].get_type();
        }
    }
}

impl IntoFunctionArg for FunctionArg {
    fn arg(&mut self) -> Result<FunctionArg, AuditError> {
        Ok(self.clone())
    }

    fn with_deref(&mut self, send_actions_ref: &std::collections::HashMap<String, usize>, send_results: &Vec<crate::audit::rule_actions::send::SendActionResults>) -> Result<FunctionArg, AuditError> {
        match self {
            FunctionArg::REF(refer) => {
                let num_id = match refer.id.parse::<usize>() {
                    Ok(num_id) => {
                        num_id
                    },
                    Err(_) => {
                        let Some(num_id) = send_actions_ref.get(&refer.id) else {
                            let err_str = format!("Cannot find any send action with id '{}'", &refer.id);
                            return Err(AuditError(err_str));
                        };

                        num_id.to_owned()
                    }
                };

                if num_id >= send_results.len() {
                    let err_str = format!("Tried to access send results by id {}, but there are only {} send actions", num_id, send_results.len());
                    return Err(AuditError(err_str));
                }

                let single_result = &send_results[num_id];

                let mut args_array: Vec<FunctionArg> = Vec::default();
                for results_per_payload in single_result.iter() {
                    for (request, _, repeated_responses) in results_per_payload.iter() {
                        match &refer.message_part {
                            MessagePart::BODY => {
                                match &refer.pair_part {
                                    PairPart::REQUEST => {
                                        args_array.push(FunctionArg::STRING(request.body.to_str_lossy().to_string()));
                                    },
                                    PairPart::RESPONSE => {
                                        for response in repeated_responses.iter() {
                                            args_array.push(FunctionArg::STRING(response.body.to_str_lossy().to_string()));
                                        }
                                    }
                                }
                            },
                            MessagePart::METHOD => {
                                match &refer.pair_part {
                                    PairPart::REQUEST => {
                                        args_array.push(FunctionArg::STRING(request.method.clone()));
                                    },
                                    PairPart::RESPONSE => {
                                        // Checks must be done in parsing time
                                        unreachable!()
                                    }
                                }
                            },
                            MessagePart::PATH => {
                                match &refer.pair_part {
                                    PairPart::REQUEST => {
                                        args_array.push(FunctionArg::STRING(request.method.clone()));
                                    },
                                    PairPart::RESPONSE => {
                                        // Checks must be done in parsing time
                                        unreachable!()
                                    }
                                }
                            },
                            MessagePart::VERSION => {
                                match &refer.pair_part {
                                    PairPart::REQUEST => {
                                        args_array.push(FunctionArg::STRING(request.version.clone()));
                                    },
                                    PairPart::RESPONSE => {
                                        for response in repeated_responses.iter() {
                                            args_array.push(FunctionArg::STRING(response.version.clone()));
                                        }
                                    }
                                }
                            },
                            MessagePart::STATUS => {
                                match &refer.pair_part {
                                    PairPart::REQUEST => {
                                        unreachable!()
                                    },
                                    PairPart::RESPONSE => {
                                        for response in repeated_responses.iter() {
                                            args_array.push(FunctionArg::STRING(response.status.clone()));
                                        }
                                    }
                                }
                            },
                            MessagePart::HEADER(header_name) => {
                                match &refer.pair_part {
                                    PairPart::REQUEST => {
                                        let header_value = request.headers.get_all(header_name)
                                            .iter()
                                            .map(|val| {
                                                val.as_bytes().to_str_lossy()
                                            })
                                            .collect::<Vec<Cow<str>>>()
                                            .join("; ");

                                        // Can be empty!
                                        args_array.push(FunctionArg::STRING(header_value));
                                    },
                                    PairPart::RESPONSE => {
                                        for response in repeated_responses.iter() {
                                            let header_value = response.headers.get_all(header_name)
                                                .iter()
                                                .map(|val| {
                                                    val.as_bytes().to_str_lossy()
                                                })
                                                .collect::<Vec<Cow<str>>>()
                                                .join("; ");

                                            // Can be empty!
                                            args_array.push(FunctionArg::STRING(header_value));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                
                return Ok(FunctionArg::SEVERAL(args_array));
            },
            _ => {
                return self.arg();
            }
        }
    }
}

impl KnownType for FunctionArg {
    fn is_boolean(&self) -> bool {
        return match self {
            FunctionArg::BOOLEAN(_) => { true },
            FunctionArg::SEVERAL(array) => {
                self.type_of_several(array) == ArgType::BOOLEAN
            },
            _ => { false }
        }
    }

    fn is_integer(&self) -> bool {
        return match self {
            FunctionArg::INTEGER(_) => { true },
            FunctionArg::SEVERAL(array) => {
                self.type_of_several(array) == ArgType::INTEGER
            },
            _ => { false }
        }
    }

    fn is_string(&self) -> bool {
        return match self {
            FunctionArg::STRING(_) => { true },
            FunctionArg::REF(_) => { true },
            FunctionArg::SEVERAL(array) => {
                self.type_of_several(array) == ArgType::STRING
            },
            _ => { false }
        }
    }

    fn get_type(&self) -> ArgType {
        return match &self {
            FunctionArg::STRING(_) => ArgType::STRING,
            FunctionArg::BOOLEAN(_) => ArgType::BOOLEAN,
            FunctionArg::INTEGER(_)  => ArgType::INTEGER,
            FunctionArg::REF(_) => ArgType::STRING,
            FunctionArg::SEVERAL(array) => {
                if array.len() == 0 {
                    return ArgType::NULL;
                }
                else {
                    return array[0].get_type();
                }
            }
            FunctionArg::NULL => ArgType::NULL
        }
    }
}
