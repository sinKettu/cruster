use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub(crate) enum ExtractionMode {
    LINE,
    MATCH
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub(crate) struct RuleGetAction {
    find_id: String,
    // This field will store more convinient representation of find_id after first check
    find_id_cache: Option<usize>,

    extract: String,
    // This field will store more convinient representation of extract after first check
    extract_cache: Option<ExtractionMode>,

    pattern: String
}