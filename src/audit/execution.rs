use std::sync::Arc;

use crossbeam::channel::{unbounded as unbounded_channel, Receiver, Sender, TryRecvError};
use log::debug;
use tokio;

use super::contexts::traits::PassiveRuleExecutionContext;
use super::contexts::PassiveRuleContext;
use super::{AuditError, AuditRule, Rule, RuleByProtocal, RuleFinalState, RuleType};
use crate::audit::contexts::traits::{ActiveRuleExecutionContext, BasicContext, WithChangeAction, WithFindAction};
use crate::audit::contexts::ActiveRuleContext;
use crate::http_storage::RequestResponsePair;


#[allow(dead_code)] // Used by the frozen YAML executor.
fn print_dbg_banner(label: &str) {
    debug!("\n");
    debug!("----------------------------------------");
    debug!("              {}", label);
    debug!("----------------------------------------");
    debug!("\n");
}


#[allow(dead_code)] // YAML execution is temporarily disabled.
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

                            let change_steps = match self.get_change_inner_action_by_index(apply_id) {
                                Ok(change_steps) => {
                                    change_steps
                                },
                                Err(err) => {
                                    let err_str = format!("with rule {} cannot execute send action with apply_id {}: {}", self.get_id(), apply_id, err.to_string());
                                    return RuleFinalState::Failed(err_str);
                                }
                            };
                            
                            debug!("SendAction - will apply change action with index {}", apply_id);
                            if let Err(err) = action.exec(&mut ctxt, change_steps).await {
                                let err_str = format!("Rule '{}' failed for pair {} on send action: {:?}", self.get_id(), pair.index, err.to_string());
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
    ScanRule((Arc<AuditRule>, Vec<Arc<RequestResponsePair>>)),
    Stop,
}

pub(crate) enum WorkerToMainMsg {
    Result(RuleFinalState),
    Error(AuditError),
    Log(String),
    Stopped
}

async fn execute_rule_with_runtime_retry(
    worker: usize,
    rule: Arc<AuditRule>,
    pairs: Vec<Arc<RequestResponsePair>>,
    tx: &Sender<WorkerToMainMsg>,
) -> Vec<(usize, RuleFinalState, Vec<super::python::PythonRuleLog>)> {
    let rule_id = rule.get_id().to_string();
    match execute_rule_stream_once(worker, rule.clone(), pairs.clone(), tx).await {
        Ok(_) => vec![],
        Err((first_crash, delivered)) => {
            let _ = tx.send(WorkerToMainMsg::Log(format!(
                "[worker {}] Python runtime thread crashed on rule '{}': {}; restarting from this rule",
                worker, rule_id, first_crash
            )));
            let remaining: Vec<_> = pairs
                .iter()
                .filter(|pair| !delivered.contains(&pair.index))
                .cloned()
                .collect();
            match execute_rule_stream_once(worker, rule, remaining.clone(), tx).await {
                Ok(_) => vec![],
                Err((second_crash, retry_delivered)) => {
                    let reason = format!(
                        "Rule '{}' crashed its Python runtime thread twice and was skipped: {}",
                        rule_id, second_crash
                    );
                    let _ = tx.send(WorkerToMainMsg::Log(format!("[worker {}] {}", worker, reason)));
                    remaining
                        .into_iter()
                        .filter(|pair| !retry_delivered.contains(&pair.index))
                        .map(|pair| (pair.index, RuleFinalState::Failed(reason.clone()), vec![]))
                        .collect()
                }
            }
        }
    }
}

async fn execute_rule_stream_once(
    worker: usize,
    rule: Arc<AuditRule>,
    pairs: Vec<Arc<RequestResponsePair>>,
    tx: &Sender<WorkerToMainMsg>,
) -> Result<Vec<usize>, (String, Vec<usize>)> {
    let rule_id = rule.get_id().to_string();
    let (result_tx, mut result_rx) = tokio::sync::mpsc::unbounded_channel();
    let execution = rule.execute_many_stream(pairs, result_tx);
    tokio::pin!(execution);
    let mut delivered = Vec::new();
    loop {
        tokio::select! {
            result = &mut execution => {
                while let Ok((pair_index, state, logs)) = result_rx.try_recv() {
                    emit_pair_result(worker, &rule_id, pair_index, state, logs, tx);
                    delivered.push(pair_index);
                }
                return match result {
                    Ok(_) => Ok(delivered),
                    Err(reason) => Err((reason, delivered)),
                };
            }
            item = result_rx.recv() => {
                if let Some((pair_index, state, logs)) = item {
                    emit_pair_result(worker, &rule_id, pair_index, state, logs, tx);
                    delivered.push(pair_index);
                }
            }
        }
    }
}

fn emit_pair_result(
    worker: usize,
    rule_id: &str,
    pair_index: usize,
    state: RuleFinalState,
    logs: Vec<super::python::PythonRuleLog>,
    tx: &Sender<WorkerToMainMsg>,
) {
    for entry in logs {
        let _ = tx.send(WorkerToMainMsg::Log(format!(
            "[rule log][{}][worker {}][rule '{}'][pair {}] {}",
            entry.level.to_uppercase(), worker, rule_id, pair_index, entry.message
        )));
    }
    let _ = tx.send(WorkerToMainMsg::Result(state));
}

async fn execute_rule_job_with_retry(
    worker: usize,
    rule: Arc<AuditRule>,
    pairs: Vec<Arc<RequestResponsePair>>,
    tx: Sender<WorkerToMainMsg>,
) -> Vec<(usize, RuleFinalState, Vec<super::python::PythonRuleLog>)> {
    let rule_id = rule.get_id().to_string();
    let first_rule = rule.clone();
    let first_pairs = pairs.clone();
    let first_tx = tx.clone();
    let first = tokio::spawn(async move {
        execute_rule_with_runtime_retry(worker, first_rule, first_pairs, &first_tx).await
    }).await;
    match first {
        Ok(results) => results,
        Err(first_crash) => {
            let _ = tx.send(WorkerToMainMsg::Log(format!(
                "[worker {}] audit rule task crashed on rule '{}': {}; restarting from this rule",
                worker, rule_id, first_crash
            )));
            let retry_pairs = pairs.clone();
            let retry_tx = tx.clone();
            match tokio::spawn(async move {
                execute_rule_with_runtime_retry(worker, rule, retry_pairs, &retry_tx).await
            }).await {
                Ok(results) => results,
                Err(second_crash) => {
                    let reason = format!(
                        "Rule '{}' crashed its audit task twice and was skipped: {}",
                        rule_id, second_crash
                    );
                    let _ = tx.send(WorkerToMainMsg::Log(format!("[worker {}] {}", worker, reason)));
                    pairs.into_iter().map(|pair| (
                        pair.index,
                        RuleFinalState::Failed(reason.clone()),
                        vec![],
                    )).collect()
                }
            }
        }
    }
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
                let _ = cloned_tx.send(WorkerToMainMsg::Log(format!("[worker {}] spawned", _i + 1)));
                loop {
                    match cloned_rx.try_recv() {
                        Ok(cmd) => {
                            match cmd {
                                MainToWorkerCmd::ScanRule(data) => {
                                    let (rule, pairs) = (data.0, data.1);
                                    let rule_id = rule.get_id().to_string();
                                    let _ = cloned_tx.send(WorkerToMainMsg::Log(format!(
                                        "[worker {}] executing rule '{}' with {} pair(s)",
                                        _i + 1, rule_id, pairs.len()
                                    )));

                                    let results = execute_rule_job_with_retry(
                                        _i + 1,
                                        rule,
                                        pairs,
                                        cloned_tx.clone(),
                                    ).await;

                                    for (pair_index, state, rule_logs) in results {
                                        for entry in rule_logs {
                                            let _ = cloned_tx.send(WorkerToMainMsg::Log(format!(
                                                "[rule log][{}][worker {}][rule '{}'][pair {}] {}",
                                                entry.level.to_uppercase(), _i + 1, rule_id, pair_index, entry.message
                                            )));
                                        }
                                        let _ = cloned_tx.send(WorkerToMainMsg::Result(state));
                                    }
                                    let _ = cloned_tx.send(WorkerToMainMsg::Log(format!("[worker {}] finished executing rule '{}'", _i + 1, rule_id)));
                                },
                                MainToWorkerCmd::Stop => {
                                    let _ = cloned_tx.send(WorkerToMainMsg::Stopped);
                                    let _ = cloned_tx.send(WorkerToMainMsg::Log(format!("[worker {}] stopped", _i + 1)));
                                    return;
                                }
                            }
                        },
                        Err(err) => {
                            if let TryRecvError::Empty = err { }
                            else {
                                let audit_err = AuditError(err.to_string());
                                let _ = cloned_tx.send(WorkerToMainMsg::Error(audit_err));
                            }
                        }
                    };
                }
            }
        );
    }

    return (main_to_workers_tx, workers_to_main_rx);
}
