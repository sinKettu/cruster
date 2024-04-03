use std::{str::FromStr, collections::HashMap};

use log::debug;

use crate::audit::contexts::traits::{WithChangeAction, WithWatchAction};

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

    pub(crate) fn exec<'pair_lt, 'rule_lt, T>(&self, ctxt: &mut T) -> Result<(), AuditError>
    where
        T: WithWatchAction<'pair_lt> + WithChangeAction<'pair_lt>
    {
        let watch_results = ctxt.watch_results();
        let wid = self.watch_id_cache.as_ref().unwrap();

        debug!("ChangeAction - watch_id '{}' resolved into index {}", &self.watch_id, wid.id);

        if wid.id >= watch_results.len() {
            let err_str = format!("action has watch_id '{}' which is resolved into watch-list element with index {} that is greater than the list size - {}", &self.watch_id, wid.id, watch_results.len());
            return Err(AuditError(err_str));
        }

        let group_name = match wid.group_name.as_ref() {
            Some(gn) => { gn },
            None => { "0" }
        };

        debug!("ChangeAction - will change capture group from WatchAction with name '{}'", group_name);

        let single_watch_result = &watch_results[wid.id];

        match single_watch_result.get(group_name) {
            Some(f) => {
                debug!("ChangeAction - found wanted capture");
                ctxt.add_change_result(Some(f.clone()));
            },
            None => {
                debug!("ChangeAction - didn't find wanted capture");
                ctxt.add_change_result(None);
            }
        }

        Ok(())
    }
}
