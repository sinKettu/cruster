use crate::{audit::{rule_actions::WatchId, types::{CapturesBorders, SendActionResultsPerPatternEntry, SingleCaptureGroupCoordinates}, AuditError, Rule, RuleResult}, cruster_proxy::request_response::{HyperRequestWrapper, HyperResponseWrapper}, http_storage::RequestResponsePair};


pub(crate) trait BasicContext<'pair_lt, 'rule_lt> {
    fn init(rule: &'rule_lt Rule, pair: &'pair_lt RequestResponsePair) -> Self;
    
    fn initial_pair(&self) -> &RequestResponsePair;
    fn initial_request(&self) -> Option<&'pair_lt HyperRequestWrapper>;
    fn initial_response(&self) -> Option<&'pair_lt HyperResponseWrapper>;
    
    fn pair_id(&self) -> usize;
    fn rule_id(&self) -> &str;
}

pub(crate) trait WithWatchAction<'pair_lt, 'rule_lt>: BasicContext<'pair_lt, 'rule_lt> {
    fn add_watch_result(&mut self, res: CapturesBorders);
    fn watch_results(&self) -> &Vec<CapturesBorders>;
}

pub(crate) trait WithChangeAction<'pair_lt, 'rule_lt>: BasicContext<'pair_lt, 'rule_lt> {
    // Result of change action is WatchId of successful watch action, that should be changed
    fn add_change_result(&mut self, res: SingleCaptureGroupCoordinates);
    fn change_results(&self) -> &Vec<SingleCaptureGroupCoordinates>;
    fn found_anything_to_change(&self) -> bool;
}

pub(crate) trait WithSendAction<'pair_lt, 'rule_lt>: BasicContext<'pair_lt, 'rule_lt> {
    fn add_send_result(&mut self, res: SendActionResultsPerPatternEntry<'rule_lt>);
    fn send_results(&self) -> &Vec<SendActionResultsPerPatternEntry>;
}

pub(crate) trait WithFindAction<'pair_lt, 'rule_lt>: BasicContext<'pair_lt, 'rule_lt> {
    fn add_find_result(&mut self, res: bool);
    fn find_results(&self) -> &Vec<bool>;
    fn found_anything(&self) -> bool;
}

pub(crate) trait WithGetAction<'pair_lt, 'rule_lt>: BasicContext<'pair_lt, 'rule_lt> {
    fn get_pair_by_id(&self, id: usize) -> Result<&SendActionResultsPerPatternEntry<'rule_lt>, AuditError>;
    fn find_action_secceeded(&self, id: usize) -> bool;
    fn add_empty_result(&mut self);
    fn add_get_result(&mut self, res: Vec<u8>);
}

pub(crate) trait ActiveRuleExecutionContext<'pair_lt, 'rule_lt>: WithWatchAction<'pair_lt, 'rule_lt> + WithChangeAction<'pair_lt, 'rule_lt> + WithSendAction<'pair_lt, 'rule_lt> + WithFindAction<'pair_lt, 'rule_lt> {
    fn make_result(self) -> RuleResult;
}
