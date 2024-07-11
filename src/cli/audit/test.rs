use std::sync::Arc;

use crossbeam::channel::TryRecvError;
use num_cpus;

use crate::{audit::{execution::{spawn_threads, MainToWorkerCmd, WorkerToMainMsg}, load_rule::compose_files_list_by_config, result::syntax_string, Rule, RuleFinalState}, cli::CrusterCLIError, config::AuditConfig, http_storage::RequestResponsePair};


pub(crate) async fn exec(_arg: &str, http_data_path: &str, aconf: &AuditConfig) -> Result<(), CrusterCLIError> {
    let tasks = match &aconf.tasks {
        Some(number) => {
            *number
        }
        None => {
            num_cpus::get()
        }
    };

    let mut storage = crate::http_storage::HTTPStorage::default();
    storage.load(http_data_path)?;
    println!("{}\n", syntax_string());

    let (tx, rx) = spawn_threads(tasks).await;

    let rule_files = compose_files_list_by_config(&aconf)?;
    let mut rules: Vec<Arc<Rule>> = Vec::with_capacity(rule_files.len());
    let pairs: Vec<Arc<RequestResponsePair>> = storage.into();

    for file_name in rule_files.iter() {
        let mut rule = Rule::from_file(&file_name)?;
        rule.check_up()?;
        rules.push(Arc::new(rule));
    }
    for rule in rules.iter() {
        for pair in pairs.iter() {
            tx.send(MainToWorkerCmd::Scan((rule.clone(), pair.clone())))?;
        }
    }

    for _ in 0..tasks {
        tx.send(MainToWorkerCmd::Stop)?;
    }

    let mut stopped_workers = 0;
    while stopped_workers != tasks {
        match rx.try_recv() {
            Ok(msg) => {
                match msg {
                    WorkerToMainMsg::Result(res) => {
                        match res {
                            RuleFinalState::Failed(reason) => {
                                println!("Failed: {}", reason);
                            },
                            RuleFinalState::Skipped(reason) => {
                                println!("Skiped: {}", reason);
                            },
                            RuleFinalState::Finished(possible_result) => {
                                match possible_result {
                                    Some(res) => {
                                        println!("{}", res);
                                    },
                                    None => {

                                    }
                                }
                            }
                        }
                    },
                    WorkerToMainMsg::Error(err) => {
                        return Err(err.into());
                    },
                    WorkerToMainMsg::Log(message) => {
                        eprintln!("{}", message);
                    },
                    WorkerToMainMsg::Stopped => {
                        stopped_workers += 1;
                    }
                }
            },
            Err(err) => {
                if let TryRecvError::Empty = err { }
                else {
                    return Err(err.into());
                }
            }
        }
    }

    Ok(())
}