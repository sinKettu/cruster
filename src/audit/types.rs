use std::collections::HashMap;

use crate::cruster_proxy::request_response::{HyperRequestWrapper, HyperResponseWrapper};

#[derive(Debug, Clone)]
pub(crate) struct SingleCoordinates {
    pub(crate) line: usize,
    pub(crate) start: usize,
    pub(crate) end: usize,
}

#[derive(Debug, Clone)]
pub(crate) struct SingleSendActionResult {
    pub(crate) request_sent: HyperRequestWrapper,
    pub(crate) positions_changed: SingleCoordinates,
    // Vector because one request may be sent multiple times
    pub(crate) responses_received: Vec<HyperResponseWrapper>,
}

// Key of the map is capture group name or number
// Value of the map is coordinates of captured substring: (line number, start, end)
pub(crate) type CapturesBorders = HashMap<String, Vec<SingleCoordinates>>;

pub(crate) type SingleCaptureGroupCoordinates = Vec<SingleCoordinates>;

// index of SSAR in this vector is index of payload in watch action
pub(crate) type PayloadsTests<'rule_lifetime> = HashMap<&'rule_lifetime str, SingleSendActionResult>;
//pub(crate) type PayloadsTests = Vec<SingleSendActionResult>;

// index of payloads tests set in this vector is index of single change action result (coordinates) in context
pub(crate) type SendActionResultsPerPatternEntry<'rule_lifetime> = Vec<PayloadsTests<'rule_lifetime>>;
