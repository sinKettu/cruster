use std::sync::Arc;

use clap::ArgMatches;
use crossbeam::channel::TryRecvError;

use crate::{
    audit::{execution::{spawn_threads, MainToWorkerCmd, WorkerToMainMsg}, load_rule::compose_files_list_by_config, result::syntax_string, Rule, RuleFinalState}, cli::CrusterCLIError, config::{AuditConfig, AuditEntities}, http_storage::RequestResponsePair
};

pub(crate) fn modify_audit_config_with_cmd_args(mut config: AuditConfig, args: &ArgMatches) -> Result<AuditConfig, CrusterCLIError> {
    if args.get_flag("active") {
        config.active = true;
    }

    if args.get_flag("passive") {
        config.passive = true;
    }

    if args.get_flag("verbose") {
        config.verbose = Some(true);
    }

    if let Some(rule_paths) = args.get_many::<String>("add-path") {
        for pth in rule_paths {
            config.rules.push(pth.clone());
        }
    }

    if let Some(rule_path) = args.get_one::<String>("rule-path") {
        config.rules = vec![rule_path.clone()];
    }

    if let Some(tags) = args.get_many::<String>("tags") {
        if config.include.is_none() {
            config.include = Some(
                AuditEntities {
                    tags: tags.map(|t| { t.to_string() }).collect(),
                    paths: vec![],
                    ids: vec![]
                }
            )
        }
        else {
            config.include.as_mut().unwrap().tags.extend(tags.map(|t| { t.to_string() }));
        }
    }

    if let Some(etags) = args.get_many::<String>("exclude-tags") {
        if config.exclude.is_none() {
            config.exclude = Some(
                AuditEntities {
                    tags: etags.map(|t| { t.to_string() }).collect(),
                    paths: vec![],
                    ids: vec![]
                }
            )
        }
        else {
            config.exclude.as_mut().unwrap().tags.extend(etags.map(|t| { t.to_string() }));
        }
    }

    if let Some(ids) = args.get_many::<String>("ids") {
        if config.include.is_none() {
            config.include = Some(
                AuditEntities {
                    tags: vec![],
                    paths: vec![],
                    ids: ids.map(|t| { t.to_string() }).collect()
                }
            )
        }
        else {
            config.include.as_mut().unwrap().ids.extend(ids.map(|t| { t.to_string() }));
        }
    }

    if let Some(eids) = args.get_many::<String>("exclude-ids") {
        if config.exclude.is_none() {
            config.exclude = Some(
                AuditEntities {
                    tags: vec![],
                    paths: vec![],
                    ids: eids.map(|t| { t.to_string() }).collect()
                }
            )
        }
        else {
            config.exclude.as_mut().unwrap().ids.extend(eids.map(|t| { t.to_string() }));
        }
    }

    Ok(config)
}

pub(crate) async fn exec(audit_conf: &AuditConfig, http_data_path: &str) -> Result<(), CrusterCLIError> {
    let tasks = match &audit_conf.tasks {
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

    let rule_files = compose_files_list_by_config(&audit_conf)?;
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
                        if let Some(verbose) = audit_conf.verbose {
                            if verbose {
                                eprintln!("{}", message);
                            }
                        }
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