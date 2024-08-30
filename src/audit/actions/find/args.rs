use std::{borrow::Cow, sync::Arc};

use bstr::ByteSlice;
use serde::{Deserialize, Serialize};

use crate::{audit::types::{OpArgWithRef, SendResultEntryRef, SingleSendResultEntry}, http_storage::RequestResponsePair};

use super::AuditError;

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub(crate) enum ExecutableExpressionArgsTypes {
    STRING,
    INTEGER,
    BOOLEAN,
    // REFERENCE
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub(crate) enum PairPart {
    REQUEST,
    RESPONSE
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub(crate) enum MessagePart {
    METHOD,
    PATH,
    VERSION,
    STATUS,
    HEADER(String),
    BODY
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub(crate) struct Reference {
    pub(crate) id: usize,
    pub(crate) pair_part: PairPart,
    pub(crate) message_part: MessagePart
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub(crate) enum ExecutableExpressionArgsValues {
    String(String),
    Integer(i64),
    Boolean(bool),
    Reference(Reference),
    Variable((String, ExecutableExpressionArgsTypes)), // (Operation Name, Operation Returning Type)
    Several(Vec<ExecutableExpressionArgsValues>),
    WithSendResReference(Arc<OpArgWithRef>)
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub(crate) struct ExecutableExpressionArg {
    pub(crate) r#type: String,
    pub(crate) value: String,

    // This field stores more convinient representation of args types
    pub(crate) type_cache: Option<ExecutableExpressionArgsValues>
}

impl ExecutableExpressionArgsValues {
    pub(crate) fn get_type(&self) -> ExecutableExpressionArgsTypes {
        match self {
            ExecutableExpressionArgsValues::Boolean(_) => ExecutableExpressionArgsTypes::BOOLEAN,
            ExecutableExpressionArgsValues::Integer(_) => ExecutableExpressionArgsTypes::INTEGER,
            ExecutableExpressionArgsValues::String(_) => ExecutableExpressionArgsTypes::STRING,
            // Reference always returns String or several Strings
            ExecutableExpressionArgsValues::Reference(_) => ExecutableExpressionArgsTypes::STRING,
            ExecutableExpressionArgsValues::Variable((_, op_type)) => op_type.clone(),
            ExecutableExpressionArgsValues::Several(array) => {
                if array.len() == 0 {
                    unreachable!("Empty set of args after dereferencing Reference. Please, make a github issue about this")
                }
                else {
                    // Developer manually ensure that all items in array has the same type
                    array[0].get_type()
                }
            },
            ExecutableExpressionArgsValues::WithSendResReference(arg) => arg.arg.get_type()
        }
    }

    pub(crate) fn boolean(&self) -> bool {
        match self {
            Self::Boolean(b) => {
                b.clone()
            },
            _ => {
                panic!("The program just tried to access argument {:?} as boolean, but it is not possible", self);
            }
        }
    }

    pub(crate) fn string(&self) -> String {
        match self {
            Self::String(s) => {
                s.clone()
            },
            _ => {
                panic!("The program just tried to access argument {:?} as string, but it is not possible", self);
            }
        }
    }

    pub(crate) fn integer(&self) -> i64 {
        match self {
            Self::Integer(i) => {
                i.clone()
            },
            _ => {
                panic!("The program just tried to access argument {:?} as integer, but it is not possible", self);
            }
        }
    }
}

impl Reference {
    pub(super) fn deref(&self, pair: &RequestResponsePair, send_results: &Vec<Vec<SingleSendResultEntry>>) -> Result<ExecutableExpressionArgsValues, AuditError> {
        // Get initial request/response
        let dereferenced: ExecutableExpressionArgsValues = if self.id == 0 {
            match &self.message_part {
                MessagePart::METHOD => {
                    ExecutableExpressionArgsValues::WithSendResReference(
                        Arc::new(
                            OpArgWithRef {
                                arg: ExecutableExpressionArgsValues::String(pair.request.as_ref().unwrap().method.clone()),
                                refer: vec![ SendResultEntryRef { send_action_id: 0, index: 0 } ],
                                one_arg: true
                            }
                        )
                    )
                },
                MessagePart::HEADER(hname) => {
                    let hmap = match self.pair_part {
                        PairPart::REQUEST => { &pair.request.as_ref().unwrap().headers },
                        PairPart::RESPONSE => { &pair.response.as_ref().unwrap().headers }
                    };

                    let values = hmap.get_all(hname)
                        .iter()
                        .map(|val| {
                            val.as_bytes().to_str_lossy()
                        })
                        .collect::<Vec<Cow<str>>>()
                        .join("; ");

                    let res = format!("{}: {}", hname, values);

                    ExecutableExpressionArgsValues::WithSendResReference(
                        Arc::new(
                            OpArgWithRef {
                                arg: ExecutableExpressionArgsValues::String(res),
                                refer: vec![ SendResultEntryRef { send_action_id: 0, index: 0 } ],
                                one_arg: true
                            }
                        )
                    )             
                },
                MessagePart::PATH => {
                    ExecutableExpressionArgsValues::WithSendResReference(
                        Arc::new(
                            OpArgWithRef {
                                arg: ExecutableExpressionArgsValues::String(pair.request.as_ref().unwrap().get_request_path()),
                                refer: vec![ SendResultEntryRef { send_action_id: 0, index: 0 } ],
                                one_arg: true
                            }
                        )
                    )
                },
                MessagePart::VERSION => {
                    let version = match self.pair_part {
                        PairPart::REQUEST => { pair.request.as_ref().unwrap().version.clone() },
                        PairPart::RESPONSE => { pair.response.as_ref().unwrap().version.clone() }
                    };

                    ExecutableExpressionArgsValues::WithSendResReference(
                        Arc::new(
                            OpArgWithRef {
                                arg: ExecutableExpressionArgsValues::String(version),
                                refer: vec![ SendResultEntryRef { send_action_id: 0, index: 0 } ],
                                one_arg: true
                            }
                        )
                    )
                },
                MessagePart::BODY => {
                    let body = match self.pair_part {
                        PairPart::REQUEST => { pair.request.as_ref().unwrap().body.to_str_lossy().to_string() },
                        PairPart::RESPONSE => { pair.response.as_ref().unwrap().body.to_str_lossy().to_string() }
                    };

                    ExecutableExpressionArgsValues::WithSendResReference(
                        Arc::new(
                            OpArgWithRef {
                                arg: ExecutableExpressionArgsValues::String(body),
                                refer: vec![ SendResultEntryRef { send_action_id: 0, index: 0 } ],
                                one_arg: true
                            }
                        )
                    )
                },
                MessagePart::STATUS => {
                    ExecutableExpressionArgsValues::WithSendResReference(
                        Arc::new(
                            OpArgWithRef {
                                arg: ExecutableExpressionArgsValues::String(pair.response.as_ref().unwrap().status.clone()),
                                refer: vec![ SendResultEntryRef { send_action_id: 0, index: 0 } ],
                                one_arg: true
                            }
                        )
                    )
                }
            }
        }
        else {
            let id = self.id;

            let mut values: Vec<ExecutableExpressionArgsValues> = Vec::default();
            let send_result = &send_results[id];
            let mut references: Vec<SendResultEntryRef> = Vec::with_capacity(send_result.len());

            for (entry_index, entry) in send_result.iter().enumerate() {
                match self.pair_part {
                    PairPart::REQUEST => {
                        let value = match &self.message_part {
                            MessagePart::METHOD => {
                                ExecutableExpressionArgsValues::String(entry.request.method.clone())
                            },
                            MessagePart::HEADER(hname) => {
                                let hmap = &entry.request.headers;
            
                                let values = hmap.get_all(hname)
                                    .iter()
                                    .map(|val| {
                                        val.as_bytes().to_str_lossy()
                                    })
                                    .collect::<Vec<Cow<str>>>()
                                    .join("; ");
            
                                let res = format!("{}: {}", hname, values);
            
                                ExecutableExpressionArgsValues::String(res)
                            },
                            MessagePart::PATH => {
                                ExecutableExpressionArgsValues::String(entry.request.get_request_path())
                            },
                            MessagePart::VERSION => {
                                let version = entry.request.version.clone();
                                ExecutableExpressionArgsValues::String(version)
                            },
                            MessagePart::BODY => {
                                let body = entry.request.body.to_str_lossy().to_string();
                                ExecutableExpressionArgsValues::String(body)
                            },
                            MessagePart::STATUS => {
                                unreachable!()
                            }
                        };

                        let reference = SendResultEntryRef {
                            send_action_id: id,
                            index: entry_index
                        };

                        references.push(reference);
                        values.push(value);
                    },
                    PairPart::RESPONSE => {
                        let response = &entry.response;
                        let value = match &self.message_part {
                            MessagePart::METHOD => {
                                unreachable!()
                            },
                            MessagePart::HEADER(hname) => {
                                let hmap = &response.headers;
            
                                let values = hmap.get_all(hname)
                                    .iter()
                                    .map(|val| {
                                        val.as_bytes().to_str_lossy()
                                    })
                                    .collect::<Vec<Cow<str>>>()
                                    .join("; ");
            
                                let res = format!("{}: {}", hname, values);
            
                                ExecutableExpressionArgsValues::String(res)
                            },
                            MessagePart::PATH => {
                                unreachable!()
                            },
                            MessagePart::VERSION => {
                                let version = response.version.clone();
                                ExecutableExpressionArgsValues::String(version)
                            },
                            MessagePart::BODY => {
                                let body = response.body.to_str_lossy().to_string();
                                ExecutableExpressionArgsValues::String(body)
                            },
                            MessagePart::STATUS => {
                                ExecutableExpressionArgsValues::String(response.status.clone())
                            }
                        };

                        let reference = SendResultEntryRef {
                            send_action_id: id,
                            index: entry_index
                        };

                        references.push(reference);
                        values.push(value);
                    }
                };
            }

            ExecutableExpressionArgsValues::WithSendResReference(
                Arc::new(
                    OpArgWithRef {
                        arg: ExecutableExpressionArgsValues::Several(values),
                        refer: references,
                        one_arg: false
                    }
                )
            )
        };


        Ok(dereferenced)
    }
}
