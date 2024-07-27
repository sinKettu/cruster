use std::{collections::HashMap, sync::Arc};

use bstr::ByteSlice;

use super::{traits::{ActiveRuleExecutionContext, BasicContext, WithChangeAction, WithFindAction, WithGetAction, WithSendAction, WithWatchAction}, ActiveRuleContext};
use crate::{audit::{types::{CapturesBorders, SerializableSendResultEntry, SingleCaptureGroupCoordinates, SingleSendResultEntry}, AuditError, Rule, RuleResult}, http_storage::RequestResponsePair};

impl<'pair_lt, 'rule_lt> BasicContext<'pair_lt> for ActiveRuleContext
{
    fn init(rule: &Rule, pair: Arc<RequestResponsePair>) -> Self {
        // set initial pair as send action result with index 0
        let initial_send_results = vec![
            vec![
                SingleSendResultEntry {
                    request: Arc::new(pair.request.clone().unwrap()),
                    payload: Arc::new("__INITIAL_PAIR__".to_string()),
                    response: pair.response.clone().unwrap()
                }
            ]
        ];

        ActiveRuleContext {
            rule_id: rule.get_id().to_string(),
            pair,
            watch_results: Vec::with_capacity(10),
            watch_succeeded_for_change: false,
            change_results: Vec::with_capacity(10),
            send_results: initial_send_results,
            find_results: Vec::with_capacity(10),
            get_result: HashMap::with_capacity(10),
        }
    }

    fn initial_pair(&self) -> &RequestResponsePair {
        &self.pair
    }

    fn initial_request(&self) -> Option<&crate::cruster_proxy::request_response::HyperRequestWrapper> {
        self.pair.request.as_ref()
    }

    fn initial_response(&self) -> Option<&crate::cruster_proxy::request_response::HyperResponseWrapper> {
        self.pair.response.as_ref()
    }

    fn pair_id(&self) -> usize {
        self.pair.index
    }

    fn rule_id(&self) -> &str {
        &self.rule_id
    }
}

impl<'pair_lt, 'rule_lt> WithWatchAction<'pair_lt> for ActiveRuleContext {
    fn add_watch_result(&mut self, res: CapturesBorders) {
        self.watch_results.push(res);
    }

    fn watch_results(&self) -> &Vec<CapturesBorders> {
        &self.watch_results
    }
}

impl<'pair_lt, 'rule_lt> WithChangeAction<'pair_lt> for ActiveRuleContext {
    fn add_change_result(&mut self, res: Option<SingleCaptureGroupCoordinates>) {
        if res.is_some() {
            self.watch_succeeded_for_change = true;
        }

        self.change_results.push(res);
    }

    fn change_results(&self) -> &Vec<Option<SingleCaptureGroupCoordinates>> {
        &self.change_results
    }

    fn found_anything_to_change(&self) -> bool {
        self.watch_succeeded_for_change
    }
}

impl<'pair_lt, 'rule_lt> WithSendAction<'pair_lt> for ActiveRuleContext {
    fn add_send_result(&mut self, res: Vec<SingleSendResultEntry>) {
        self.send_results.push(res);
    }

    fn send_results(&self) -> &Vec<Vec<SingleSendResultEntry>> {
        &self.send_results
    }
}

impl<'pair_lt, 'rule_lt> WithFindAction<'pair_lt> for ActiveRuleContext {
    fn add_find_result(&mut self, res: (bool, Option<usize>)) {
        self.find_results.push(res);
    }

    fn find_results(&self) -> &Vec<(bool, Option<usize>)> {
        &self.find_results
    }

    fn found_anything(&self) -> bool {
        self.find_results.iter().any(|result| { result.0 })
    }
}

impl<'pair_lt, 'rule_lt> WithGetAction<'pair_lt> for ActiveRuleContext {
    fn find_action_secceeded(&self, id: usize) -> bool {
        if id >= self.find_results.len() {
            false
        }
        else {
            self.find_results[id].0
        }
    }

    fn get_pair_by_id(&self, id: usize) -> Result<&Vec<SingleSendResultEntry>, AuditError> {
        if id >= self.send_results.len() {
            return Err(AuditError("Index of Send results is out of bounds".to_string()));
        }

        Ok(&self.send_results[id])
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

impl<'pair_lt, 'rule_lt> ActiveRuleExecutionContext<'pair_lt> for ActiveRuleContext {
    fn make_result(self, rule: &Rule) -> crate::audit::RuleResult {
        
        let mut findings: HashMap<String, (Vec<String>, Vec<SerializableSendResultEntry>)> = HashMap::with_capacity(self.find_results().len());
        for (index, find_result) in self.find_results().iter().enumerate() {
            if find_result.0 {
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

                let mut serializable_entries = vec![];
                if let Some(send_index) = find_result.1 {
                    let send_results = self.send_results();
                    let active_rule = rule.http_active_rule().unwrap();
                    let required_send_results = active_rule.find[index].get_required_send_results();

                    for res_index in required_send_results {
                        if *res_index == 0 {
                            continue;
                        }

                        let send_res_entry = &send_results[*res_index][send_index];
                        let request = send_res_entry.request.to_string();
                        let payload = send_res_entry.payload.as_str().to_string();
                        let response = send_res_entry.response.to_string();

                        let serializable_res_entry = SerializableSendResultEntry {
                            request,
                            payload,
                            response
                        };

                        serializable_entries.push(serializable_res_entry);
                    }
                }
                
                findings.insert(find_id, (extracted_data, serializable_entries));
            }
        }

        RuleResult {
            id: 0,
            rule_id: rule.id.clone(),
            about: rule.metadata.about.to_string(),
            protocol: "http".to_string(),
            r#type: "active".to_string(),
            pair_index: self.pair_id(),
            severity: rule.severity.clone(),
            findings,
            initial_request: self.initial_request().unwrap().clone().to_string(),
            initial_response: self.initial_response().unwrap().clone().to_string()
        }
    }
}
