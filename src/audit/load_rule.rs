use std::fs::File;

use super::AuditError;
use super::Rule;
use crate::config::AuditConfig;

use log::debug;
use serde_yaml as yml;

impl Rule {
    pub(crate) fn from_file(filename: &str) -> Result<Rule, AuditError> {
        let rule_file = match std::fs::OpenOptions::new().read(true).open(filename) {
            Ok(rule_file) => { rule_file },
            Err(err) => {
                return Err(AuditError(err.to_string()));
            }
        };
        
        let rule: Rule = match yml::from_reader(rule_file) {
            Ok(rule) => {
                rule
            },
            Err(err) => {
                return Err(AuditError(format!("Unable to parse '{}': {}", filename, err.to_string())));
            }
        };

        Ok(rule)
    }
}

fn recursively_walk_dir(dir_path: &std::path::Path, base_dir: String) -> Vec<String> {
    let mut result = Vec::new();
    let entities = dir_path.read_dir();
    
    if let Err(err) = entities {
        debug!("Error while trying to walk directory {}: {}", dir_path.to_string_lossy(), err);
        return result;
    }

    for entry in entities.unwrap() {
        match entry {
            Ok(entry) => {
                if entry.path().is_file() {
                    let os_file_name = entry.file_name();
                    let fln = os_file_name.to_string_lossy();

                    if fln.ends_with(".yaml") || fln.ends_with(".yml") {
                        result.push(format!("{}/{}", base_dir, fln.clone()));
                    }
                } else if entry.path().is_dir() {
                    let os_file_name = entry.file_name();
                    let fln = os_file_name.to_string_lossy();

                    let mut sub_results = recursively_walk_dir(&entry.path(), format!("{}/{}", base_dir, fln));
                    result.append(&mut sub_results);
                }
            },
            Err(err) => {
                debug!("Error while walking directory {}: {}", dir_path.to_string_lossy(), err);
            }
        }
    }

    return result;
}

pub(crate) fn compose_files_list_by_config(audit_conf: &AuditConfig) -> Result<Vec<String>, AuditError> {
    // TODO: include/exclude
    if ! (audit_conf.active || audit_conf.passive) {
        debug!("All audit types are disabled");
        return Err(AuditError("Cannot compose list of rules because all types of rules are disabled".to_string()));
    }

    let mut result: Vec<String> = Vec::default();
    for raw_rule_path in audit_conf.rules.iter() {
        let rule_path = std::path::Path::new(raw_rule_path);
        if ! rule_path.exists() {
            debug!("File or directory does not exist: {}", raw_rule_path);
            continue;
        }

        if rule_path.is_file() {
            if raw_rule_path.ends_with(".yaml") || raw_rule_path.ends_with(".yml") {
                result.push(raw_rule_path.clone());
            }
        } else if rule_path.is_dir() {
            let mut walk_results = recursively_walk_dir(rule_path, raw_rule_path.clone());
            result.append(&mut walk_results);
        } else {
            return Err(AuditError(format!("Cannot work with path '{}'", raw_rule_path)));
        }
    }

    Ok(result)
}
