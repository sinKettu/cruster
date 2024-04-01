use std::sync::Arc;

use crossbeam::channel::{unbounded as unbounded_channel, Receiver, Sender, TryRecvError};
use log::debug;
use tokio;

use super::contexts::traits::PassiveRuleExecutionContext;
use super::contexts::PassiveRuleContext;
use super::{AuditError, Rule, RuleByProtocal, RuleFinalState, RuleType};
use crate::audit::contexts::traits::{ActiveRuleExecutionContext, BasicContext, WithChangeAction, WithFindAction};
use crate::audit::contexts::ActiveRuleContext;
use crate::http_storage::RequestResponsePair;


impl Rule {
    pub(crate) async fn execute<'pair_lt, 'rule_lt>(&'rule_lt self, pair: &'pair_lt RequestResponsePair) -> RuleFinalState {
        match &self.rule {
            RuleByProtocal::Http(rule_type) => {
                debug!("Rule protocol is HTTP");
                match rule_type {
                    RuleType::Active(actions) => {
                        debug!("Rule type is active");
                        let mut ctxt: ActiveRuleContext = ActiveRuleContext::init(self, pair);

                        // WATCH
                        debug!("Watch actions: {}", actions.watch.len());
                        for action in actions.watch.iter() {
                            debug!("Executing watch action: {:?}", action);
                            if let Err(err) = action.exec(&mut ctxt) {
                                let err_str = format!("Rule '{}' failed for pair {} on watch action: {}", self.get_id(), pair.index, err);
                                return RuleFinalState::Failed(err_str)
                            }
                            debug!("");
                        }
                        debug!("Watch actions finished");

                        // CHANGE
                        debug!("Change actions: {}", actions.change.len());
                        for action in actions.change.iter() {
                            debug!("Executing change action: {:?}", action);
                            if let Err(err) = action.exec(&mut ctxt) {
                                let err_str = format!("Rule '{}' failed for pair {} on change action: {}", self.get_id(), pair.index, err);
                                return RuleFinalState::Failed(err_str)
                            }
                            debug!("");
                        }
                        debug!("Change actions finished");

                        if ! ctxt.found_anything_to_change() {
                            debug!("No watch action succeeded to use it in change action, rule finished");
                            let reason = format!("With rule '{}' for pair {} no patterns have matched", self.get_id(), pair.index);
                            return RuleFinalState::Skipped(reason);
                        }

                        // SEND
                        debug!("Send actions: {}", actions.send.len());
                        for action in actions.send.iter() {
                            debug!("Executing send action: {:?}", action);
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

                            debug!("SendAction - will apply change action with index {}, payloads ({}) will be placed in the following way: {:?}", apply_id, payloads.len(), placement);

                            if let Err(err) = action.exec(&mut ctxt, placement, payloads).await {
                                let err_str = format!("Rule '{}' failed for pair {} on send action: {}", self.get_id(), pair.index, err);
                                return RuleFinalState::Failed(err_str)
                            }
                            debug!("");
                        }

                        // FIND
                        debug!("Find actions: {}", actions.find.len());
                        for action in actions.find.iter() {
                            debug!("Executing find action: {:?}", action);
                            if let Err(err) = action.exec(&mut ctxt) {
                                let err_str = format!("Rule '{}' failed for pair {} on find action: {}", self.get_id(), pair.index, err);
                                return RuleFinalState::Failed(err_str)
                            }
                            debug!("");
                        }

                        if ! ctxt.found_anything() {
                            debug!("Could not find anything with this rule and pair, rule finished");
                            return RuleFinalState::Finished(None);
                        }

                        // GET
                        if let Some(get_actions) = actions.get.as_ref() {
                            debug!("Get actions: {}", get_actions.len());
                            for action in get_actions {
                                debug!("Executing get action: {:?}", action);
                                if let Err(err) = action.exec(&mut ctxt) {
                                    let err_str = format!("Rule '{}' failed for pair {} on get action: {}", self.get_id(), pair.index, err);
                                    return RuleFinalState::Failed(err_str)
                                }
                                debug!("");
                            }
                        }
                        else {
                            debug!("No Get actions in this rule");
                        }

                        return RuleFinalState::Finished(Some(ctxt.make_result(&self)));
                    },
                    RuleType::Passive(actions) => {
                        debug!("Rule type is passive");
                        let mut ctxt: PassiveRuleContext = PassiveRuleContext::init(self, pair);
                        
                        // FIND
                        debug!("Find actions: {}", actions.find.len());
                        for action in actions.find.iter() {
                            debug!("Executing find action: {:?}", action);
                            if let Err(err) = action.exec(&mut ctxt) {
                                let err_str = format!("Rule '{}' failed for pair {} on find action: {}", self.get_id(), pair.index, err);
                                return RuleFinalState::Failed(err_str)
                            }
                            debug!("");
                        }

                        if ! ctxt.found_anything() {
                            debug!("Could not find anything with this rule and pair, rule finished");
                            return RuleFinalState::Finished(None);
                        }

                        // GET
                        if let Some(get_actions) = actions.get.as_ref() {
                            debug!("Get actions: {}", get_actions.len());
                            for action in get_actions {
                                debug!("Executing get action: {:?}", action);
                                if let Err(err) = action.exec(&mut ctxt) {
                                    let err_str = format!("Rule '{}' failed for pair {} on get action: {}", self.get_id(), pair.index, err);
                                    return RuleFinalState::Failed(err_str)
                                }
                                debug!("");
                            }
                        }
                        else {
                            debug!("No Get actions in this rule");
                        }

                        return RuleFinalState::Finished(Some(ctxt.make_result(&self)));
                    }
                }
            }
        }
    }
}


pub(crate) enum MainToWorkerCmd {
    Start,
    Scan((Arc<Rule>, Arc<RequestResponsePair>)),
    Stop,
}

pub(crate) enum WorkerToMainMsg {
    Result(RuleFinalState),
    Error(AuditError),
    Stopped
}


pub(crate) async fn spawn_threads(num: usize) -> (Sender<MainToWorkerCmd>, Receiver<WorkerToMainMsg>) {
    let (main_to_workers_tx, main_to_workers_rx) = unbounded_channel::<MainToWorkerCmd>();
    let (workers_to_main_tx, workers_to_main_rx) = unbounded_channel::<WorkerToMainMsg>();

    // use arc to store rules and pairs
    for _i in 0..num {
        let cloned_tx = workers_to_main_tx.clone();
        let cloned_rx = main_to_workers_rx.clone();

        tokio::spawn(
            async move {
                loop {
                    match cloned_rx.try_recv() {
                        Ok(cmd) => {
                            match cmd {
                                MainToWorkerCmd::Scan(data) => {
                                    let (rule, pair) = (data.0, data.1);
                                    // println!("Thread {} received rule={} and pair={}", _i, rule.get_id(), pair.index);
                                    let state = rule.execute(&pair).await;
                                    cloned_tx.send(WorkerToMainMsg::Result(state)).unwrap();
                                },
                                MainToWorkerCmd::Stop => {
                                    cloned_tx.send(WorkerToMainMsg::Stopped).unwrap();
                                    return;
                                },
                                MainToWorkerCmd::Start => {
                                    // println!("Task {} started", _i);
                                }
                            }
                        },
                        Err(err) => {
                            if let TryRecvError::Empty = err { }
                            else {
                                let audit_err = AuditError(err.to_string());
                                cloned_tx.send(WorkerToMainMsg::Error(audit_err)).unwrap();
                            }
                        }
                    };
                }
            }
        );
    }

    return (main_to_workers_tx, workers_to_main_rx);
}