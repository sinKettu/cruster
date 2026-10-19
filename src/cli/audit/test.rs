use std::sync::Arc;


use num_cpus;

use crate::{audit::{execution::{execute_scan, WorkerToMainMsg}, load_rule, result::syntax_string, RuleFinalState}, cli::CrusterCLIError, config::AuditConfig, http_storage::RequestResponsePair};


pub(crate) async fn exec(
    _arg: &str,
    http_data_path: &str,
    aconf: &AuditConfig,
) -> Result<(), CrusterCLIError> {
    let tasks = match &aconf.tasks {
        Some(number) => *number,
        None => num_cpus::get(),
    };
    if tasks == 0 {
        return Err(CrusterCLIError::from("Audit workers count cannot be zero"));
    }

    let mut storage = crate::http_storage::HTTPStorage::default();
    storage.load(http_data_path)?;
    println!("{}\n", syntax_string());

    let rules = load_rule::load_rules(aconf)?;
    let pairs: Vec<Arc<RequestResponsePair>> = storage.into();
    execute_scan(rules, pairs, tasks, |msg| {
        match msg {
            WorkerToMainMsg::Result(res) => match res.state {
                RuleFinalState::Failed(reason) => {
                    println!("Failed: {}", reason);
                }
                RuleFinalState::Skipped(reason) => {
                    println!("Skiped: {}", reason);
                }
                RuleFinalState::Finished(possible_result) => match possible_result {
                    Some(res) => {
                        println!("{}", res);
                    }
                    None => {}
                },
            },
            WorkerToMainMsg::Error(err) => {
                return Err(err.into());
            }
            WorkerToMainMsg::Log(message) => {
                eprintln!("{}", message);
            }
            WorkerToMainMsg::Stopped => {}
        }
        Ok::<_, CrusterCLIError>(())
    })
    .await
}
