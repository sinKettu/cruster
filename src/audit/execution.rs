use std::sync::Arc;

use crossbeam::channel::{unbounded as unbounded_channel, Receiver, Sender, TryRecvError};
use tokio;

use super::{Rule, AuditError, RuleFinalState};
use crate::audit::contexts::traits::{ActiveRuleExecutionContext, BasicContext, WithChangeAction, WithFindAction};
use crate::audit::contexts::ActiveRuleContext;
use crate::http_storage::RequestResponsePair;


impl Rule {
    pub(crate) async fn execute<'pair_lt, 'rule_lt>(&'rule_lt self, pair: &'pair_lt RequestResponsePair) -> RuleFinalState {
        match self.r#type {
            crate::audit::RuleType::Active => {
                let mut ctxt: ActiveRuleContext = ActiveRuleContext::init(self, pair);

                // WATCH
                for action in self.rule.watch.as_ref().unwrap().iter() {
                    if let Err(err) = action.exec(&mut ctxt) {
                        let err_str = format!("Rule '{}' failed for pair {} on watch action: {}", self.get_id(), pair.index, err);
                        return RuleFinalState::Failed(err_str)
                    }
                }

                // CHANGE
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

                // SEND
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
                        let err_str = format!("Rule '{}' failed for pair {} on send action: {}", self.get_id(), pair.index, err);
                        return RuleFinalState::Failed(err_str)
                    }
                }

                // FIND
                for action in self.rule.find.as_ref().unwrap() {
                    if let Err(err) = action.exec(&mut ctxt) {
                        let err_str = format!("Rule '{}' failed for pair {} on find action: {}", self.get_id(), pair.index, err);
                        return RuleFinalState::Failed(err_str)
                    }
                }

                if ! ctxt.found_anything() {
                    return RuleFinalState::Finished(None);
                }

                // GET
                if let Some(get_actions) = self.rule.get.as_ref() {
                    for action in get_actions {
                        if let Err(err) = action.exec(&mut ctxt) {
                            let err_str = format!("Rule '{}' failed for pair {} on get action: {}", self.get_id(), pair.index, err);
                            return RuleFinalState::Failed(err_str)
                        }
                    }    
                }

                return RuleFinalState::Finished(Some(ctxt.make_result(&self)));
            },
            crate::audit::RuleType::Passive => {
                todo!()
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


pub(crate) async fn spawn_threads<'a, 'b>(num: usize) -> (Sender<MainToWorkerCmd>, Receiver<WorkerToMainMsg>) {
    let (main_to_workers_tx, main_to_workers_rx) = unbounded_channel::<MainToWorkerCmd>();
    let (workers_to_main_tx, workers_to_main_rx) = unbounded_channel::<WorkerToMainMsg>();

    // use arc to store rules and pairs
    for _i in 0..num {
        let cloned_tx = workers_to_main_tx.clone();
        let cloned_rx = main_to_workers_rx.clone();

        tokio::spawn(
            async move {
                println!("Spawned worker {}", _i);
                loop {
                    match cloned_rx.try_recv() {
                        Ok(cmd) => {
                            match cmd {
                                MainToWorkerCmd::Scan(data) => {
                                    let (rule, pair) = (data.0, data.1);
                                    // println!("Thread {} received rule={} and pair={}", _i, rule.get_id(), pair.index);
                                    let state = rule.execute(&pair).await;
                                    if let Err(err) = cloned_tx.send(WorkerToMainMsg::Result(state)) {
                                        println!("Task {} Error: {}", _i, err);
                                    }
                                },
                                MainToWorkerCmd::Stop => {
                                    println!("Finished task {}", _i);
                                    cloned_tx.send(WorkerToMainMsg::Stopped).unwrap();
                                    return;
                                },
                                MainToWorkerCmd::Start => {
                                    println!("Task {} started", _i);
                                }
                            }
                        },
                        Err(err) => {
                            if let TryRecvError::Empty = err { }
                            else {
                                let audit_err = AuditError(err.to_string());
                                if let Err(err) = cloned_tx.send(WorkerToMainMsg::Error(audit_err)) {
                                    println!("Task error on receiving command: {}", err);
                                }
                            }
                        }
                    };
                }
            }
        );
    }

    return (main_to_workers_tx, workers_to_main_rx);
}