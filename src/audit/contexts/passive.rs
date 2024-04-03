use std::{collections::HashMap, sync::Arc};

use bstr::ByteSlice;

use super::{traits::{ActiveRuleExecutionContext, BasicContext, PassiveRuleExecutionContext, WithChangeAction, WithFindAction, WithGetAction, WithSendAction, WithWatchAction}, ActiveRuleContext, PassiveRuleContext};
use crate::{audit::{actions::WatchId, types::{CapturesBorders, SendActionResultsPerPatternEntry, SingleCaptureGroupCoordinates, SingleCoordinates, SingleSendActionResult}, AuditError, Rule, RuleResult}, http_storage::RequestResponsePair};

impl<'pair_lt, 'rule_lt> BasicContext<'pair_lt> for PassiveRuleContext<'pair_lt>
{
    fn init(rule: &Rule, pair: &'pair_lt RequestResponsePair) -> Self {
        // set initial pair as send action result with index 0
        let initial_send_result: Vec<SendActionResultsPerPatternEntry> = vec![
            vec![
                HashMap::from([
                    (
                        Arc::new("__VERY_INITIAL_PAIR__".to_string()),
                        SingleSendActionResult {
                            request_sent: pair.request.as_ref().unwrap().clone(),
                            positions_changed: SingleCoordinates {
                                line: 0,
                                start: 0,
                                end: 0
                            },
                            responses_received: vec![pair.response.as_ref().unwrap().clone()]
                        }
                    )
                ])
            ]
        ];

        PassiveRuleContext {
            rule_id: rule.get_id().to_string(),
            pair,
            initial_send_result,
            find_results: Vec::with_capacity(10),
            get_result: HashMap::with_capacity(10)
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

// There are actually no send actions in passive rule, but this trait must be implemented
impl<'pair_lt, 'rule_lt> WithSendAction<'pair_lt> for PassiveRuleContext<'pair_lt> {
    fn add_send_result(&mut self, res: crate::audit::types::SendActionResultsPerPatternEntry) {
        unreachable!("method .add_send_result() must not be used with passive context, something goes wrong")
    }

    fn send_results(&self) -> &Vec<crate::audit::types::SendActionResultsPerPatternEntry> {
        &self.initial_send_result
    }
}


impl<'pair_lt, 'rule_lt> WithFindAction<'pair_lt> for PassiveRuleContext<'pair_lt> {
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

impl<'pair_lt, 'rule_lt> WithGetAction<'pair_lt> for PassiveRuleContext<'pair_lt> {
    fn find_action_secceeded(&self, id: usize) -> bool {
        if id >= self.find_results.len() {
            false
        }
        else {
            self.find_results[id]
        }
    }

    fn get_pair_by_id(&self, id: usize) -> Result<&SendActionResultsPerPatternEntry, AuditError> {
        if id != 0 {
            return Err(AuditError("index of request/response in passive may only be 0".to_string()));
        }

        Ok(&self.initial_send_result[0])
    }

    fn add_empty_result(&mut self, find_action_index: usize) {
        if ! self.get_result.contains_key(&find_action_index) {
            self.get_result.insert(find_action_index, Vec::with_capacity(10));
        }
    }

    fn add_get_result(&mut self, find_action_index: usize, res: Vec<u8>) {
        if let Some(array) = self.get_result.get_mut(&find_action_index) {
            array.push(res);
        }
        else {
            let mut array = Vec::with_capacity(10);
            array.push(res);
            self.get_result.insert(find_action_index, array);
        }
    }
}

impl<'pair_lt, 'rule_lt> PassiveRuleExecutionContext<'pair_lt> for PassiveRuleContext<'pair_lt> {
    fn make_result(self, rule: &Rule) -> RuleResult {
        let mut findings = HashMap::with_capacity(self.find_results.len());
        for (index, find_result) in self.find_results.iter().enumerate() {
            if *find_result {
                let find_id = rule.get_find_action_str_id(index).unwrap();
                let extracted_data = if let Some(one_get_result) = self.get_result.get(&index) {
                    one_get_result
                        .iter()
                        .map(|v| { v.as_slice().to_str_lossy().to_string() })
                        .collect::<Vec<String>>()
                }
                else {
                    Vec::default()
                };
                
                findings.insert(find_id, extracted_data);
            }
        }

        RuleResult {
            rule_id: rule.id.clone(),
            pair_index: self.pair_id(),
            severity: rule.severity.clone(),
            findings
        }
    }
}