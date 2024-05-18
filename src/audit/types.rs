use std::{collections::HashMap, rc::Rc, sync::Arc};

use serde::{Deserialize, Serialize};

use crate::cruster_proxy::request_response::{HyperRequestWrapper, HyperResponseWrapper};

use super::actions;

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
pub(crate) type PayloadsTests = HashMap<Arc<String>, SingleSendActionResult>;
//pub(crate) type PayloadsTests = Vec<SingleSendActionResult>;

// index of payloads tests set in this vector is index of single change action result (coordinates) in context
pub(crate) type SendActionResultsPerPatternEntry = Vec<PayloadsTests>;

#[derive(Clone, Debug)]
pub(crate) struct SingleSendResultEntry {
    pub(crate) request: Arc<HyperRequestWrapper>,
    pub(crate) payload: Arc<String>,
    pub(crate) response: HyperResponseWrapper
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub(crate) struct SerializableSendResultEntry {
    pub(crate) request: String,
    pub(crate) payload: String,
    pub(crate) response: String
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub(crate) struct SendResultEntryRef {
    pub(crate) send_action_id: usize,
    pub(crate) index: usize
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub(crate) struct OpArgWithRef {
    pub(crate) arg: actions::find::args::ExecutableExpressionArgsValues,
    pub(crate) refer: Vec<SendResultEntryRef>,
    pub(crate) one_arg: bool
}