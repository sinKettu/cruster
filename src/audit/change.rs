use std::{str::FromStr, collections::HashMap};

use serde::{Serialize, Deserialize};

use super::AuditError;

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub(crate) struct WatchId {
    id: usize,
    group_name: Option<String>
}

impl WatchId {
    pub(crate) fn new(id: usize, group_name: Option<String>) -> Self {
        WatchId {
            id,
            group_name
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub(crate) enum ChangeValuePlacement {
    BEFORE,
    AFTER,
    REPLACE
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub(crate) struct RuleChangeAction {
    id: Option<String>,

    watch_id: String,
    // This field will store more convinient representation of watch_id after first check
    watch_id_cache: Option<WatchId>,

    placement: String,
    // This field will store more convinient representation of placement after first check
    placement_cache: Option<ChangeValuePlacement>,

    values: Vec<String>,
}

impl RuleChangeAction {
    pub(crate) fn check_up(&mut self, watch_id_ref: &HashMap<String, usize>) -> Result<(), AuditError> {
        let splitted_watch_id: Vec<&str> = self.watch_id.split(".").collect();
        if splitted_watch_id.len() > 2 {
            let err_mes = format!("Wwrong format of .watch_id, it should have 0 or 1 '.' (dot) delimiter: {}", &self.watch_id);
            return Err(AuditError::from_str(&err_mes).unwrap());
        }

        let parsed_watch_id: WatchId = match splitted_watch_id.len() {
            2 => {
                if let Ok(num_id) = splitted_watch_id[0].parse::<usize>() {
                    WatchId::new(num_id, Some(splitted_watch_id[1].to_string()))
                }
                else {
                    if let Some(num_id) = watch_id_ref.get(splitted_watch_id[0]) {
                        WatchId::new(num_id.to_owned(), Some(splitted_watch_id[1].to_string()))
                    }
                    else {
                        return Err(AuditError(format!("Cannot find watch action with id '{}'", splitted_watch_id[0])));
                    }
                }
            },
            1 => {
                if let Some(num_id) = watch_id_ref.get(splitted_watch_id[0]) {
                    WatchId::new(num_id.to_owned(), None)
                }
                else {
                    return Err(AuditError(format!("Cannot find watch action with id '{}'", splitted_watch_id[0])));
                }
            },
            _ => {
                let err_mes = format!("Wwrong format of .watch_id, it should have 0 or 1 '.' (dot) delimiter and must not be empty: {}", &self.watch_id);
            return Err(AuditError::from_str(&err_mes).unwrap());
            }
        };

        self.watch_id_cache = Some(parsed_watch_id);

        match self.placement.to_lowercase().as_str() {
            "before" => {
                self.placement_cache = Some(ChangeValuePlacement::BEFORE)
            },
            "after" => {
                self.placement_cache = Some(ChangeValuePlacement::AFTER)
            },
            "replace" => {
                self.placement_cache = Some(ChangeValuePlacement::REPLACE)
            },
            _ => {
                return Err(AuditError(format!("Wrong placement parameter, must be before/after/replace: {}", &self.placement)));
            }
        }

        Ok(())
    }

    pub(crate) fn get_id(&self) -> Option<String> {
        self.id.clone()
    }
}
