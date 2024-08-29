pub(crate) mod active;
pub(crate) mod passive;
pub(crate) mod traits;

use std::{collections::HashMap, sync::Arc};

use active::AllChangeStepsResults;

use crate::http_storage::RequestResponsePair;

use super::types::{CapturesBorders, SingleSendResultEntry};

pub(crate) struct ActiveRuleContext {
    rule_id: String,
    pair: Arc<RequestResponsePair>,

    watch_results: Vec<CapturesBorders>,

    watch_succeeded_for_change: bool,
    change_results: Vec<AllChangeStepsResults>,

    send_results: Vec<Vec<SingleSendResultEntry>>,

    find_results: Vec<(bool, Option<usize>)>,

    get_result: HashMap<usize, Vec<Vec<u8>>>,
}

pub(crate) struct PassiveRuleContext {
    rule_id: String,
    pair: Arc<RequestResponsePair>,

    initial_send_result: Vec<Vec<SingleSendResultEntry>>,
    find_results: Vec<(bool, Option<usize>)>,
    get_result: HashMap<usize, Vec<Vec<u8>>>,
}