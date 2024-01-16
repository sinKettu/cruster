use serde::{Deserialize, Serialize};

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
    pub(crate) id: String,
    pub(crate) pair_part: PairPart,
    pub(crate) message_part: MessagePart
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub(crate) enum ExecutableExpressionArgsValues {
    String(String),
    Integer(i64),
    Boolean(bool),
    Reference(Reference),
    // (Operation Name, Operation Returning Type)
    Variable((String, ExecutableExpressionArgsTypes)),
    Several(Vec<ExecutableExpressionArgsValues>)
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
            }
        }
    }
}