use super::{Rule, AuditError, RuleFinalState};
use crate::audit::rule_actions::ReqResCoordinates;
use crate::audit::rule_contexts::traits::{BasicContext, WithChangeAction, WithFindAction};
use crate::audit::rule_contexts::ActiveRuleContext;
use crate::http_storage::RequestResponsePair;


impl Rule {
    pub(crate) async fn execute<'pair_lt, 'rule_lt>(&'rule_lt self, pair: &'pair_lt RequestResponsePair) -> RuleFinalState {
        match self.r#type {
            crate::audit::RuleType::Active => {
                let mut ctxt: ActiveRuleContext = ActiveRuleContext::init(self, pair);

                for action in self.rule.watch.as_ref().unwrap().iter() {
                    if let Err(err) = action.exec(&mut ctxt) {
                        let err_str = format!("Rule '{}' failed for pair {} on watch action: {}", self.get_id(), pair.index, err);
                        return RuleFinalState::Failed(err_str)
                    }
                }

                for action in self.rule.change.as_ref().unwrap() {
                    if let Err(err) = action.exec(&mut ctxt) {
                        let err_str = format!("Rule '{}' failed for pair {} on change action: {}", self.get_id(), pair.index, err);
                        return RuleFinalState::Failed(err_str)
                    }
                }

                if ! ctxt.found_anything_to_change() {
                    let reason = format!("With rule '{}' for pair {} no patterns have matched", self.get_id(), pair.index);
                    return RuleFinalState::Skipped(reason);
                }

                for action in self.rule.send.as_ref().unwrap() {
                    let apply_id = action.get_apply_id();

                    let placement = match self.get_change_placement_by_index(apply_id)  {
                        Ok(placement) => {
                            placement
                        },
                        Err(err) => {
                            let err_str = format!("Rule '{}' failed for pair {} on send action: {}", self.get_id(), pair.index, err);
                            return RuleFinalState::Failed(err_str);
                        }
                    };

                    let payloads = match self.get_payloads_by_index(apply_id) {
                        Ok(payloads) => {
                            payloads
                        },
                        Err(err) => {
                            let err_str = format!("Rule '{}' failed for pair {} on send action: {}", self.get_id(), pair.index, err);
                            return RuleFinalState::Failed(err_str);
                        }
                    };

                    if let Err(err) = action.exec(&mut ctxt, placement, payloads).await {
                        let err_str = format!("Rule '{}' failed for pair {} on watch action: {}", self.get_id(), pair.index, err);
                        return RuleFinalState::Failed(err_str)
                    }
                }

                for action in self.rule.find.as_ref().unwrap() {
                    if let Err(err) = action.exec(&mut ctxt) {
                        let err_str = format!("Rule '{}' failed for pair {} on find action: {}", self.get_id(), pair.index, err);
                        return RuleFinalState::Failed(err_str)
                    }
                }

                if ! ctxt.found_anything() {
                    return RuleFinalState::Finished(None);
                }

                for action in self.rule.get.as_ref().unwrap() {
                    if let Err(err) = action.exec(&mut ctxt) {
                        let err_str = format!("Rule '{}' failed for pair {} on get action: {}", self.get_id(), pair.index, err);
                        return RuleFinalState::Failed(err_str)
                    }
                }
            },
            crate::audit::RuleType::Passive => {
                todo!()
            }
        }

        todo!()
    }
}

pub(crate) async fn execute_one(rule: &Rule, pair: &RequestResponsePair) -> Result<RuleFinalState, AuditError> {

    todo!()
}