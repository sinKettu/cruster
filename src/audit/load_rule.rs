use std::sync::Arc;

use regex::Regex;

use super::AuditError;
use super::Rule;
use super::RuleByProtocal;
use super::RuleType;
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
        
        let mut rule: Rule = match yml::from_reader(rule_file) {
            Ok(rule) => {
                rule
            },
            Err(err) => {
                return Err(AuditError(format!("Unable to parse '{}': {}", filename, err.to_string())));
            }
        };

        rule.check_up()?;
        Ok(rule)
    }
}

impl AuditConfig {
    fn str_vec_to_re_vec(strs: &[String]) -> Result<Option<Vec<Regex>>, AuditError> {
        let mut result: Vec<Regex> = vec![];
        for pth in strs {
            let re = Regex::new(pth)?;
            result.push(re);
        }

        if result.is_empty() {
            Ok(None)
        }
        else {
            Ok(Some(result))
        }
    }

    fn get_include_paths_re(&self) -> Result<Option<Vec<Regex>>, AuditError> {
        if let Some(includies) = self.include.as_ref() {
            AuditConfig::str_vec_to_re_vec(&includies.paths)
        }
        else {
            Ok(None)
        }
    }

    fn get_exclude_paths_re(&self) -> Result<Option<Vec<Regex>>, AuditError> {
        if let Some(excludies) = self.exclude.as_ref() {
            AuditConfig::str_vec_to_re_vec(&excludies.paths)
        }
        else {
            Ok(None)
        }
    }

    fn excludes_rule(&self, rule: &Rule) -> bool {
        let rule_id = rule.get_id();
        let rule_tags = rule.metadata.tags.as_slice();

        match &rule.rule {
            RuleByProtocal::Http(http_rule) => {
                match http_rule {
                    RuleType::Active(_) => {
                        if !self.active {
                            return true;
                        }
                    },
                    RuleType::Passive(_) => {
                        if !self.passive {
                            return true;
                        }
                    }
                }
            }
        }

        if let Some(includies) = self.include.as_ref() {
            // if rule_id is not in included ids, then skip it
            if !includies.ids.is_empty() && !includies.ids.iter().any(|id| { id == rule_id }) {
                return true;
            }

            if !includies.tags.is_empty() {
                let tag_found = includies.tags
                    .iter()
                    .any(|inc_tag| {
                        rule_tags.contains(inc_tag)
                    });
                
                if !tag_found {
                    return true;
                }
            }
        }

        if let Some(excludies) = self.exclude.as_ref() {
            if !excludies.ids.is_empty() && excludies.ids.iter().any(|id| { id == rule_id} ) {
                return true;
            }

            if !excludies.tags.is_empty() {
                let tag_found = excludies.tags
                    .iter()
                    .any(|exc_tag| {
                        rule_tags.contains(exc_tag)
                    });
                
                if tag_found {
                    return true;
                }
            }            
        }

        return false;
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

pub(crate) fn load_rules(audit_conf: &AuditConfig) -> Result<Vec<Arc<Rule>>, AuditError> {
    let rules_paths = compose_files_list_by_config(audit_conf)?;
    let inc_paths = audit_conf.get_include_paths_re()?;
    let exc_paths = audit_conf.get_exclude_paths_re()?;
    let mut result: Vec<Arc<Rule>> = vec![];

    for path in rules_paths.iter() {
        debug!("Loading rule - {}", path);

        if let Some(inc_re) = inc_paths.as_ref() {
            let any_re = inc_re.iter().any(|re| { re.is_match(path) });
            if !any_re {
                debug!("Rule excluded because was not found in include paths list");
            }
        }

        if let Some(exc_re) = exc_paths.as_ref() {
            for re in exc_re {
                if re.is_match(&path) {
                    debug!("Rule excluded by path regex: {}", re.as_str());
                    continue;
                }
            }
        }

        let rule = Rule::from_file(path)?;

        if audit_conf.excludes_rule(&rule) {
            continue;
        }

        result.push(Arc::new(rule));
    }

    return Ok(result);
}