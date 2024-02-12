use std::{str::FromStr, collections::HashMap};

use crate::audit::{rule_contexts::traits::RuleExecutionContext, types::SingleCoordinates};

use super::*;

impl RuleChangeAction {
    pub(crate) fn check_up(&mut self, watch_id_ref: &HashMap<String, usize>) -> Result<(), AuditError> {
        let splitted_watch_id: Vec<&str> = self.watch_id.split(".").collect();
        if splitted_watch_id.len() > 2 {
            let err_mes = format!("Wwrong format of .watch_id, it should have 0 or 1 '.' (dot) delimiter: {}", &self.watch_id);
            return Err(AuditError::from_str(&err_mes).unwrap());
        }

        // TODO: Do not use numbers in groups names
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

    pub(crate) fn exec<'pair_lt, 'rule_lt, T: RuleExecutionContext<'pair_lt, 'rule_lt>>(&self, ctxt: &mut T) -> Result<(), AuditError> {
        let watch_results = ctxt.watch_results();
        let wid = self.watch_id_cache.as_ref().unwrap();

        if wid.id >= watch_results.len() {
            let err_str = format!("Rule has watch_id '{}' which is resolved into watch-list element with index {} that is greater than the list size - {}", &self.watch_id, wid.id, watch_results.len());
            return Err(AuditError(err_str));
        }

        let group_name = match wid.group_name.as_ref() {
            Some(gn) => { gn },
            None => { "0" }
        };

        let single_watch_result = &watch_results[wid.id];

        match single_watch_result.get(group_name) {
            Some(f) => {
                ctxt.add_change_result(f.clone());
                Ok(())
            },
            None => {
                Ok(())
            }
        }
    }
}
