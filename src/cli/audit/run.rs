use std::{fs::File, sync::Arc};

use clap::ArgMatches;
use crossbeam::channel::TryRecvError;
use regex::Regex;

use crate::cli::http::show::matches_the_filter;

use crate::{
    audit::{execution::{spawn_threads, MainToWorkerCmd, WorkerToMainMsg}, load_rule, result::{syntax_string, WriteResult}, RuleFinalState}, cli::CrusterCLIError, config::{AuditConfig, AuditEntities, Config}, http_storage::RequestResponsePair
};

pub(crate) fn modify_audit_config_with_cmd_args(mut config: AuditConfig, args: &ArgMatches) -> Result<AuditConfig, CrusterCLIError> {
    if let Some(audit_name) = args.get_one::<String>("audit-name") {
        config.name = Some(audit_name.clone());
    }

    if args.get_flag("active") {
        config.active = true;
    }

    if args.get_flag("passive") {
        config.passive = true;
    }

    if args.get_flag("verbose") {
        config.verbose = Some(true);
    }

    if let Some(filters) = args.get_many::<String>("filter") {
        config.filter.get_or_insert_with(Vec::new).extend(filters.cloned());
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

fn compile_pair_filters(config: &AuditConfig) -> Result<Vec<Regex>, CrusterCLIError> {
    config.filter.as_deref().unwrap_or_default().iter().map(|pattern| {
        Regex::new(pattern).map_err(|err| {
            CrusterCLIError::from(format!("Invalid audit HTTP pair filter '{}': {}", pattern, err))
        })
    }).collect()
}

fn filter_pairs(pairs: &mut Vec<Arc<RequestResponsePair>>, filters: &[Regex]) {
    if !filters.is_empty() {
        pairs.retain(|pair| filters.iter().any(|re| matches_the_filter(pair, re)));
    }
}

pub(crate) async fn exec(config: &Config, audit_conf: &AuditConfig, http_data_path: &str) -> Result<(), CrusterCLIError> {
    let filters = compile_pair_filters(audit_conf)?;
    let tasks = match &audit_conf.tasks {
        Some(number) => {
            *number
        }
        None => {
            num_cpus::get()
        }
    };
    if tasks == 0 {
        return Err(CrusterCLIError::from("Audit workers count cannot be zero"));
    }

    let mut storage = crate::http_storage::HTTPStorage::default();
    storage.load(http_data_path)?;
    println!("{}\n", syntax_string());

    let mut pairs: Vec<Arc<RequestResponsePair>> = storage.into();
    filter_pairs(&mut pairs, &filters);

    let audit_name = match &audit_conf.name {
        Some(name) => {
            name
        },
        None => {
            return Err(CrusterCLIError::from("Current audit does not have any name, so cannot be launched"));
        }
    };

    let audit_file = config.get_audit_results_file(audit_name);
    if let Err(err) = File::create(&audit_file) {
        return Err(CrusterCLIError::from(format!("Cannot open file to write result: {}", err)));
    }

    let (tx, rx) = spawn_threads(tasks).await;

    let rules = load_rule::load_rules(&audit_conf)?;

    if audit_conf.verbose.unwrap_or(false) {
        let jobs = rules.len().saturating_mul(pairs.len());
        eprintln!(
            "Audit plan: {} worker(s), {} rule(s), {} HTTP pair(s), {} scan job(s)",
            tasks,
            rules.len(),
            pairs.len(),
            jobs
        );
        if rules.is_empty() {
            eprintln!("Warning: no Python audit rules were loaded");
        }
        if pairs.is_empty() {
            eprintln!("Warning: no HTTP pairs selected for audit (history is empty or no pairs match the filters)");
        }
    }

    for rule in rules.iter() {
        tx.send(MainToWorkerCmd::ScanRule((rule.clone(), pairs.clone())))?;
    }
    for _ in 0..tasks {
        tx.send(MainToWorkerCmd::Stop)?;
    }

    let mut stopped_workers = 0;
    let mut finding_index = 0;
    while stopped_workers != tasks {
        match rx.try_recv() {
            Ok(msg) => {
                match msg {
                    WorkerToMainMsg::Result(res) => {
                        match res {
                            RuleFinalState::Failed(reason) => {
                                eprintln!("[{}] {}", "FAILED", reason);
                            },
                            RuleFinalState::Skipped(reason) => {
                                if let Some(verbose) = audit_conf.verbose {
                                    if verbose {
                                        eprintln!("Skiped: {}", reason);
                                    }
                                }
                            },
                            RuleFinalState::Finished(possible_result) => {
                                match possible_result {
                                    Some(res) => {
                                        let mut res = res;
                                        res.set_id(finding_index);
                                        finding_index += 1;
                                        
                                        println!("{}", res);
                                        
                                        if let Err(err) = res.write_result(&audit_file) {
                                            eprintln!("could not save last audit result: {}", err.to_string());
                                        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cruster_proxy::request_response::{HyperRequestWrapper, HyperResponseWrapper};

    fn pair() -> Arc<RequestResponsePair> {
        let mut request_headers = hyper::HeaderMap::new();
        request_headers.insert("x-request-marker", "request-header".parse().unwrap());
        let mut response_headers = hyper::HeaderMap::new();
        response_headers.insert("x-response-marker", "response-header".parse().unwrap());
        Arc::new(RequestResponsePair {
            request: Some(HyperRequestWrapper {
                method: "POST".into(),
                uri: "https://example.com/target".into(),
                version: "HTTP/1.1".into(),
                headers: request_headers,
                body: b"request-body".to_vec(),
            }),
            response: Some(HyperResponseWrapper {
                version: "HTTP/1.1".into(),
                status: "201 Created".into(),
                headers: response_headers,
                body: b"response-body".to_vec(),
            }),
            index: 7,
            timestamp: None,
        })
    }

    fn filters(patterns: &[&str]) -> Vec<Regex> {
        let config = AuditConfig {
            filter: Some(patterns.iter().map(|pattern| pattern.to_string()).collect()),
            ..AuditConfig::default()
        };
        compile_pair_filters(&config).unwrap()
    }

    #[test]
    fn audit_filter_searches_each_http_part_using_existing_matcher() {
        for pattern in [
            "^POST https://example.com/target HTTP/1.1",
            "^x-request-marker: request-header$",
            "^request-body$",
            "^HTTP/1.1 201 Created",
            "^x-response-marker: response-header$",
            "^response-body$",
        ] {
            let mut pairs = vec![pair()];
            filter_pairs(&mut pairs, &filters(&[pattern]));
            assert_eq!(pairs.len(), 1, "Pattern should match: {}", pattern);
        }
    }

    #[test]
    fn audit_filters_are_or_combined_and_remove_only_nonmatching_pairs() {
        let selected = pair();
        let mut other = (*selected).clone();
        other.index = 8;
        other.request.as_mut().unwrap().body = b"other".to_vec();
        let mut pairs = vec![selected.clone(), Arc::new(other)];
        filter_pairs(&mut pairs, &filters(&["not-present", "^request-body$"]));
        assert_eq!(pairs.len(), 1);
        assert!(Arc::ptr_eq(&pairs[0], &selected));

        filter_pairs(&mut pairs, &filters(&["not-present", "also-absent"]));
        assert!(pairs.is_empty());
    }

    #[test]
    fn audit_filters_handle_missing_request_or_response() {
        let mut request_only = (*pair()).clone();
        request_only.response = None;
        let mut response_only = (*pair()).clone();
        response_only.request = None;
        let mut empty = response_only.clone();
        empty.response = None;
        let mut pairs = vec![Arc::new(request_only), Arc::new(response_only), Arc::new(empty)];
        filter_pairs(&mut pairs, &filters(&["body"]));
        assert_eq!(pairs.len(), 2);
    }

    #[test]
    fn audit_filter_yaml_is_optional_and_round_trips() {
        let yaml = "active: false\npassive: true\nrules: []\n";
        for suffix in ["", "filter: null\n", "filter: []\n"] {
            let config: AuditConfig = serde_yaml::from_str(&format!("{}{}", yaml, suffix)).unwrap();
            let mut pairs = vec![pair()];
            filter_pairs(&mut pairs, &compile_pair_filters(&config).unwrap());
            assert_eq!(pairs.len(), 1);
        }
        let config: AuditConfig = serde_yaml::from_str(&format!(
            "{}filter:\n  - '^POST '\n  - 'response-body'\n", yaml
        )).unwrap();
        assert_eq!(config.filter.as_ref().unwrap().len(), 2);
        let serialized = serde_yaml::to_string(&config).unwrap();
        assert_eq!(config, serde_yaml::from_str(&serialized).unwrap());
    }

    #[test]
    fn audit_filter_rejects_invalid_regex_even_after_valid_pattern() {
        let config = AuditConfig {
            filter: Some(vec![".*".into(), "[".into()]),
            ..AuditConfig::default()
        };
        let error: String = compile_pair_filters(&config).unwrap_err().into();
        assert!(error.contains("Invalid audit HTTP pair filter '['"));
    }

    #[test]
    fn audit_filter_cli_patterns_append_to_yaml_patterns() {
        let matches = crate::config::parse_cmd_from([
            "cruster", "audit", "run", "selected", "-f", "request-body",
            "--filter", "response-body",
        ].iter().map(std::ffi::OsString::from).collect());
        let args = matches.subcommand_matches("cli").unwrap()
            .subcommand_matches("audit").unwrap()
            .subcommand_matches("run").unwrap();
        let config = AuditConfig {
            filter: Some(vec!["yaml-pattern".into()]),
            ..AuditConfig::default()
        };
        let config = modify_audit_config_with_cmd_args(config, args).unwrap();
        assert_eq!(config.filter.unwrap(), ["yaml-pattern", "request-body", "response-body"]);
    }
}
