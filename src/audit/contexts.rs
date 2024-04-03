pub(crate) mod active;
pub(crate) mod passive;
pub(crate) mod traits;

use std::collections::HashMap;

use crate::http_storage::RequestResponsePair;

use super::types::{CapturesBorders, SendActionResultsPerPatternEntry, SingleCaptureGroupCoordinates};

pub(crate) struct ActiveRuleContext<'pair_lt> {
    rule_id: String,
    pair: &'pair_lt RequestResponsePair,

    watch_results: Vec<CapturesBorders>,

    watch_succeeded_for_change: bool,
    change_results: Vec<Option<SingleCaptureGroupCoordinates>>,

    send_results: Vec<SendActionResultsPerPatternEntry>,

    find_results: Vec<bool>,

    get_result: HashMap<usize, Vec<Vec<u8>>>,
}

pub(crate) struct PassiveRuleContext<'pair_lt> {
    rule_id: String,
    pair: &'pair_lt RequestResponsePair,

    initial_send_result: Vec<SendActionResultsPerPatternEntry>,
    find_results: Vec<bool>,
    get_result: HashMap<usize, Vec<Vec<u8>>>,
}