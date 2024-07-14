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


fn print_dbg_banner(label: &str) {
    debug!("\n");
    debug!("----------------------------------------");
    debug!("              {}", label);
    debug!("----------------------------------------");
    debug!("\n");
}


impl Rule {
    pub(crate) async fn execute<'pair_lt, 'rule_lt>(&'rule_lt self, pair: Arc<RequestResponsePair>) -> RuleFinalState {
        match &self.rule {
            RuleByProtocal::Http(rule_type) => {
                debug!("Rule protocol is HTTP");
                match rule_type {
                    RuleType::Active(actions) => {
                        debug!("Rule type is active");
                        let mut ctxt: ActiveRuleContext = ActiveRuleContext::init(self, pair.clone());

                        // WATCH
                        print_dbg_banner("WATCH");
                        debug!("Watch actions: {}\n", actions.watch.len());
                        for action in actions.watch.iter() {
                            debug!("Executing watch action: {:?}", action);
                            if let Err(err) = action.exec(&mut ctxt) {
                                let err_str = format!("Rule '{}' failed for pair {} on watch action: {}", self.get_id(), pair.index, err.to_string());
                                return RuleFinalState::Failed(err_str)
                            }
                            debug!("");
                        }
                        debug!("Watch actions finished");

                        // CHANGE
                        print_dbg_banner("CHANGE");
                        debug!("Change actions: {}\n", actions.change.len());
                        for action in actions.change.iter() {
                            debug!("Executing change action: {:?}", action);
                            if let Err(err) = action.exec(&mut ctxt) {
                                let err_str = format!("Rule '{}' failed for pair {} on change action: {}", self.get_id(), pair.index, err.to_string());
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
                        print_dbg_banner("SEND");
                        debug!("Send actions: {}\n", actions.send.len());
                        for action in actions.send.iter() {
                            debug!("Executing send action: {:?}", action);
                            let apply_id = action.get_apply_id();

                            let inner_action = match self.get_change_inner_action_by_index(apply_id) {
                                Ok(inner_action) => {
                                    inner_action
                                },
                                Err(err) => {
                                    let err_str = format!("with rule {} cannot execute send action with apply_id {}: {}", self.get_id(), apply_id, err.to_string());
                                    return RuleFinalState::Failed(err_str);
                                }
                            };
                            
                            debug!("SendAction - will apply change action with index {}", apply_id);
                            if let Err(err) = action.exec(&mut ctxt, inner_action).await {
                                let err_str = format!("Rule '{}' failed for pair {} on send action: {}", self.get_id(), pair.index, err.to_string());
                                return RuleFinalState::Failed(err_str)
                            }
                            debug!("");
                        }

                        // FIND
                        print_dbg_banner("FIND");
                        debug!("Find actions: {}\n", actions.find.len());
                        for action in actions.find.iter() {
                            debug!("Executing find action: {:#?}\n", action);
                            if let Err(err) = action.exec(&mut ctxt) {
                                let err_str = format!("Rule '{}' failed for pair {} on find action: {}", self.get_id(), pair.index, err.to_string());
                                return RuleFinalState::Failed(err_str)
                            }
                            debug!("");
                        }

                        if ! ctxt.found_anything() {
                            debug!("Could not find anything with this rule and pair, rule finished");
                            return RuleFinalState::Finished(None);
                        }

                        // GET
                        print_dbg_banner("GET");
                        if let Some(get_actions) = actions.get.as_ref() {
                            debug!("Get actions: {}\n", get_actions.len());
                            for action in get_actions {
                                debug!("Executing get action: {:?}", action);
                                if let Err(err) = action.exec(&mut ctxt) {
                                    let err_str = format!("Rule '{}' failed for pair {} on get action: {}", self.get_id(), pair.index, err.to_string());
                                    return RuleFinalState::Failed(err_str)
                                }
                                debug!("");
                            }
                        }
                        else {
                            debug!("No Get actions in this rule");
                        }

                        // debug!("Context:\n\n{:?}", &ctxt.find_results());

                        return RuleFinalState::Finished(Some(ctxt.make_result(&self)));
                    },
                    RuleType::Passive(actions) => {
                        debug!("Rule type is passive");
                        let mut ctxt: PassiveRuleContext = PassiveRuleContext::init(self, pair.clone());
                        
                        // FIND
                        debug!("Find actions: {}", actions.find.len());
                        for action in actions.find.iter() {
                            debug!("Executing find action: {:?}", action);
                            if let Err(err) = action.exec(&mut ctxt) {
                                let err_str = format!("Rule '{}' failed for pair {} on find action: {}", self.get_id(), pair.index, err.to_string());
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
                                    let err_str = format!("Rule '{}' failed for pair {} on get action: {}", self.get_id(), pair.index, err.to_string());
                                    return RuleFinalState::Failed(err_str)
                                }
                                debug!("");
                            }
                        }
                        else {
                            debug!("No Get actions in this rule");
                        }

                        // debug!("Context:\n\n{:?}", &ctxt.find_results());

                        return RuleFinalState::Finished(Some(ctxt.make_result(&self)));
                    }
                }
            }
        }
    }
}


pub(crate) enum MainToWorkerCmd {
    Scan((Arc<Rule>, Arc<RequestResponsePair>)),
    Stop,
}

pub(crate) enum WorkerToMainMsg {
    Result(RuleFinalState),
    Error(AuditError),
    Log(String),
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
                cloned_tx.send(WorkerToMainMsg::Log(format!("[{}] Worker spawned", _i))).unwrap();
                loop {
                    match cloned_rx.try_recv() {
                        Ok(cmd) => {
                            match cmd {
                                MainToWorkerCmd::Scan(data) => {
                                    let (rule, pair) = (data.0, data.1);

                                    let uri = if let Some(request) = pair.request.as_ref() {
                                        request.uri.clone()
                                    }
                                    else {
                                        "Request is missing".to_string()
                                    };
                                    cloned_tx.send(WorkerToMainMsg::Log(format!("[{}] Worker executing rule '{}' with pair {} - {}", _i, rule.get_id(), pair.index, uri))).unwrap();

                                    let state: RuleFinalState = rule.execute(pair).await;
                                    cloned_tx.send(WorkerToMainMsg::Log(format!("[{}] Worker finished executing rule", _i))).unwrap();
                                    cloned_tx.send(WorkerToMainMsg::Result(state)).unwrap();
                                },
                                MainToWorkerCmd::Stop => {
                                    cloned_tx.send(WorkerToMainMsg::Stopped).unwrap();
                                    cloned_tx.send(WorkerToMainMsg::Log(format!("[{}] Worker finished", _i))).unwrap();
                                    return;
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