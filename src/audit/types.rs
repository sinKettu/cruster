use std::{collections::HashMap, sync::Arc};

use serde::{Deserialize, Serialize};

use crate::cruster_proxy::request_response::{HyperRequestWrapper, HyperResponseWrapper};

use super::actions;

#[derive(Debug, Clone)]
#[allow(dead_code)] // Capture coordinates for frozen YAML rules.
pub(crate) struct SingleCoordinates {
    pub(crate) line: usize,
    pub(crate) start: usize,
    pub(crate) end: usize,
}

// Key of the map is capture group name or number
// Value of the map is coordinates of captured substring: (line number, start, end)
#[allow(dead_code)] // Capture coordinates for frozen YAML rules.
pub(crate) type CapturesBorders = HashMap<String, Vec<SingleCoordinates>>;

#[allow(dead_code)] // Capture coordinates for frozen YAML rules.
pub(crate) type SingleCaptureGroupCoordinates = Vec<SingleCoordinates>;

#[derive(Clone, Debug)]
#[allow(dead_code)] // In-memory send results for frozen YAML rules.
pub(crate) struct SingleSendResultEntry {
    pub(crate) request: Arc<HyperRequestWrapper>,
    pub(crate) payloads: Vec<Arc<String>>,
    pub(crate) response: HyperResponseWrapper
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub(crate) struct SerializableSendResultEntry {
    pub(crate) request: String,
    pub(crate) payloads: Vec<String>,
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
