use std::sync::Arc;

use crate::{audit::{actions::WatchId, types::{CapturesBorders, SendActionResultsPerPatternEntry, SendResultEntryRef, SingleCaptureGroupCoordinates, SingleSendResultEntry}, AuditError, Rule, RuleResult}, cruster_proxy::request_response::{HyperRequestWrapper, HyperResponseWrapper}, http_storage::RequestResponsePair};


pub(crate) trait BasicContext<'pair_lt> {
    fn init(rule: &Rule, pair: Arc<RequestResponsePair>) -> Self;
    
    fn initial_pair(&self) -> &RequestResponsePair;
    fn initial_request(&self) -> Option<&HyperRequestWrapper>;
    fn initial_response(&self) -> Option<&HyperResponseWrapper>;
    
    fn pair_id(&self) -> usize;
    fn rule_id(&self) -> &str;
}

pub(crate) trait WithWatchAction<'pair_lt>: BasicContext<'pair_lt> {
    fn add_watch_result(&mut self, res: CapturesBorders);
    fn watch_results(&self) -> &Vec<CapturesBorders>;
}

pub(crate) trait WithChangeAction<'pair_lt>: BasicContext<'pair_lt> {
    // Result of change action is WatchId of successful watch action, that should be changed
    fn add_change_result(&mut self, res: Option<SingleCaptureGroupCoordinates>);
    fn change_results(&self) -> &Vec<Option<SingleCaptureGroupCoordinates>>;
    fn found_anything_to_change(&self) -> bool;
}

pub(crate) trait WithSendAction<'pair_lt>: BasicContext<'pair_lt> {
    fn add_send_result(&mut self, res: Vec<SingleSendResultEntry>);
    fn send_results(&self) -> &Vec<Vec<SingleSendResultEntry>>;
}

pub(crate) trait WithFindAction<'pair_lt>: BasicContext<'pair_lt> {
    fn add_find_result(&mut self, res: (bool, Option<usize>));
    fn find_results(&self) -> &Vec<(bool, Option<usize>)>;
    fn found_anything(&self) -> bool;
}

pub(crate) trait WithGetAction<'pair_lt>: BasicContext<'pair_lt> {
    fn get_pair_by_id(&self, id: usize) -> Result<&Vec<SingleSendResultEntry>, AuditError>;
    fn find_action_secceeded(&self, id: usize) -> bool;
    fn add_empty_result(&mut self, find_action_index: usize);
    fn add_get_result(&mut self, find_action_index: usize, res: Vec<u8>);
}

pub(crate) trait ActiveRuleExecutionContext<'pair_lt>: WithWatchAction<'pair_lt> + WithChangeAction<'pair_lt> + WithSendAction<'pair_lt> + WithFindAction<'pair_lt> + WithGetAction<'pair_lt> {
    fn make_result(self, rule: &Rule) -> RuleResult;
}

pub(crate) trait PassiveRuleExecutionContext<'pair_lt>: WithSendAction<'pair_lt> + WithFindAction<'pair_lt> {
    fn make_result(self, rule: &Rule) -> RuleResult;
}
