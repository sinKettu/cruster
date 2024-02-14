use super::{traits::{ActiveRuleExecutionContext, BasicContext, WithChangeAction, WithFindAction, WithSendAction, WithWatchAction}, ActiveRuleContext};
use crate::{audit::{rule_actions::WatchId, types::CapturesBorders, Rule}, http_storage::RequestResponsePair};

impl<'pair_lt, 'rule_lt> BasicContext<'pair_lt, 'rule_lt> for ActiveRuleContext<'pair_lt, 'rule_lt>
{
    fn init(rule: &'rule_lt Rule, pair: &'pair_lt RequestResponsePair) -> Self {
        ActiveRuleContext {
            rule_id: rule.get_id().to_string(),
            pair,
            watch_results: Vec::with_capacity(10),
            watch_succeeded_for_change: false,
            change_results: Vec::with_capacity(10),
            send_results: Vec::with_capacity(10),
            find_results: Vec::with_capacity(10),
        }
    }

    fn initial_pair(&self) -> &'pair_lt RequestResponsePair {
        self.pair
    }

    fn initial_request(&self) -> Option<&'pair_lt crate::cruster_proxy::request_response::HyperRequestWrapper> {
        self.pair.request.as_ref()
    }

    fn initial_response(&self) -> Option<&'pair_lt crate::cruster_proxy::request_response::HyperResponseWrapper> {
        self.pair.response.as_ref()
    }

    fn pair_id(&self) -> usize {
        self.pair.index
    }

    fn rule_id(&self) -> &str {
        &self.rule_id
    }
}

impl<'pair_lt, 'rule_lt> WithWatchAction<'pair_lt, 'rule_lt> for ActiveRuleContext<'pair_lt, 'rule_lt> {
    fn add_watch_result(&mut self, res: CapturesBorders) {
        self.watch_results.push(res);
    }

    fn watch_results(&self) -> &Vec<CapturesBorders> {
        &self.watch_results
    }
}

impl<'pair_lt, 'rule_lt> WithChangeAction<'pair_lt, 'rule_lt> for ActiveRuleContext<'pair_lt, 'rule_lt> {
    fn add_change_result(&mut self, res: crate::audit::types::SingleCaptureGroupCoordinates) {
        self.watch_succeeded_for_change = true;
        self.change_results.push(res);
    }

    fn change_results(&self) -> &Vec<crate::audit::types::SingleCaptureGroupCoordinates> {
        &self.change_results
    }

    fn found_anything_to_change(&self) -> bool {
        self.watch_succeeded_for_change
    }
}

impl<'pair_lt, 'rule_lt> WithSendAction<'pair_lt, 'rule_lt> for ActiveRuleContext<'pair_lt, 'rule_lt> {
    fn add_send_result(&mut self, res: crate::audit::types::SendActionResultsPerPatternEntry<'rule_lt>) {
        self.send_results.push(res);
    }

    fn send_results(&self) -> &Vec<crate::audit::types::SendActionResultsPerPatternEntry> {
        &self.send_results
    }
}

impl<'pair_lt, 'rule_lt> WithFindAction<'pair_lt, 'rule_lt> for ActiveRuleContext<'pair_lt, 'rule_lt> {
    fn add_find_result(&mut self, res: bool) {
        self.find_results.push(res);
    }

    fn find_results(&self) -> &Vec<bool> {
        &self.find_results
    }

    fn found_anything(&self) -> bool {
        self.find_results.iter().any(|result| { *result })
    }
}

impl<'pair_lt, 'rule_lt> ActiveRuleExecutionContext<'pair_lt, 'rule_lt> for ActiveRuleContext<'pair_lt, 'rule_lt> {
    fn make_result(self) -> crate::audit::RuleResult {
        todo!()
    }
}

// impl<'pair_lt, 'rule_lt,> RuleExecutionContext<'pair_lt, 'rule_lt> for ActiveRuleContext<'pair_lt, 'rule_lt> {
//     fn make_result(self) -> crate::audit::RuleResult {
//         todo!()
//     }

//     fn initial_pair(&self) -> &'pair_lt RequestResponsePair {
//         self.pair
//     }

//     fn initial_request(&self) -> Option<&'pair_lt crate::cruster_proxy::request_response::HyperRequestWrapper> {
//         self.pair.request.as_ref()
//     }

//     fn initial_response(&self) -> Option<&'pair_lt crate::cruster_proxy::request_response::HyperResponseWrapper> {
//         self.pair.response.as_ref()
//     }

//     fn pair_id(&self) -> usize {
//         self.pair.index
//     }

//     fn add_watch_result(&mut self, res: CapturesBorders) {
//         self.watch_results.push(res);
//     }

//     fn watch_results(&self) -> &Vec<CapturesBorders> {
//         &self.watch_results
//     }

//     fn add_change_result(&mut self, res: crate::audit::types::SingleCaptureGroupCoordinates) {
//         self.change_results.push(res);
//     }

//     fn change_results(&self) -> &Vec<crate::audit::types::SingleCaptureGroupCoordinates> {
//         &self.change_results
//     }

//     fn found_anything_to_change(&self) -> bool {
//         self.watch_succeeded_for_change
//     }

//     fn add_send_result(&mut self, res: crate::audit::types::SendActionResultsPerPatternEntry<'rule_lt>) {
//         self.send_results.push(res);
//     }

//     fn send_results(&self) -> &Vec<crate::audit::types::SendActionResultsPerPatternEntry> {
//         &self.send_results
//     }

//     fn add_find_result(&mut self, res: bool) {
//         self.find_results.push(res);
//     }

//     fn find_results(&self) -> &Vec<bool> {
//         &self.find_results
//     }
// }