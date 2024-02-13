pub(crate) mod active_rule_context;
pub(crate) mod traits;

use crate::http_storage::RequestResponsePair;

use super::types::{CapturesBorders, SendActionResultsPerPatternEntry, SingleCaptureGroupCoordinates};

pub(crate) struct ActiveRuleContext<'pair_lt, 'rule_lt> {
    rule_id: String,
    pair: &'pair_lt RequestResponsePair,

    watch_results: Vec<CapturesBorders>,

    watch_succeeded_for_change: bool,
    change_results: Vec<SingleCaptureGroupCoordinates>,

    send_results: Vec<SendActionResultsPerPatternEntry<'rule_lt>>,

    find_results: Vec<bool>,
}

