use crate::{audit::{rule_actions::WatchId, types::{CapturesBorders, SendActionResultsPerPatternEntry, SingleCaptureGroupCoordinates}, RuleResult}, cruster_proxy::request_response::{HyperRequestWrapper, HyperResponseWrapper}, http_storage::RequestResponsePair};


// TODO: split into different traits
pub(crate) trait RuleExecutionContext<'pair_lt, 'rule_lt> {
    fn make_result(self) -> RuleResult;
    fn initial_pair(&self) -> &'pair_lt RequestResponsePair;
    fn initial_request(&self) -> Option<&'pair_lt HyperRequestWrapper>;
    fn initial_response(&self) -> Option<&'pair_lt HyperResponseWrapper>;
    fn pair_id(&self) -> usize;

    fn add_watch_result(&mut self, res: CapturesBorders);
    fn watch_results(&self) -> &Vec<CapturesBorders>;

    // Result of change action is WatchId of successful watch action, that should be changed
    fn add_change_result(&mut self, res: SingleCaptureGroupCoordinates);
    fn change_results(&self) -> &Vec<SingleCaptureGroupCoordinates>;

    fn found_anything_to_change(&self) -> bool;

    fn add_send_result(&mut self, res: SendActionResultsPerPatternEntry<'rule_lt>);
    fn send_results(&self) -> &Vec<SendActionResultsPerPatternEntry>;

    fn add_find_result(&mut self, res: bool);
    fn find_results(&self) -> &Vec<bool>;
}