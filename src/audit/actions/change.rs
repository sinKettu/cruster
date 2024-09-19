use std::{collections::HashMap, str::FromStr};

use http::{header::HeaderName, HeaderValue};
use log::debug;
use base64;

use crate::{audit::contexts::{active::{AllChangeStepsResults, ChangeStepResult}, traits::{WithChangeAction, WithWatchAction}}, http_storage::serializable::Header};

use super::*;

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub(crate) struct ChangeModify {
    watch_id: String,
    placement: ChangeValuePlacement,
    values: Vec<Arc<String>>,

    watch_id_cache: Option<WatchId>, // This field will store more convinient representation of watch_id after first check
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub(crate) enum ChangeAdd {
    HEADER(Header)
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub(crate) enum InnerChangeAction {
    MODIFY(ChangeModify),
    ADD(Vec<ChangeAdd>),
}

impl ChangeModify {
    pub(crate) fn get_placement(&self) -> &ChangeValuePlacement {
        &self.placement
    }

    pub(crate) fn get_payloads(&self) -> &Vec<Arc<String>> {
        &self.values
    }
}

impl Header {
    pub(crate) fn http_header(&self) -> Result<(HeaderName, HeaderValue), AuditError> {
        let name = HeaderName::from_str(&self.key)?;
        let value = match self.encoding.as_str() {
            "utf-8" => {
                match HeaderValue::from_bytes(self.value.as_bytes()) {
                    Ok(value) => {
                        value
                    },
                    Err(err) => {
                        return Err(AuditError(err.to_string()));
                    }
                }
            },
            "base64" => {
                let data = base64::decode(&self.value)?;
                HeaderValue::from_bytes(&data)?
            },
            _ => {
                let err_msg = format!("Unknown header encoding type in change action: {}", &self.encoding);
                return Err(AuditError::from_str(&err_msg).unwrap());
            }
        };

        return Ok((name, value));
    }
}

impl RuleChangeAction {
    pub(crate) fn get_inner_action(&self) -> &Vec<InnerChangeAction> {
        &self.changes
    }

    // pub(crate) fn get_placement(&self) -> Result<ChangeValuePlacement, AuditError> {
    //     match &self.r#type {
    //         InnerChangeAction::ADD(_) => {
    //             return Err(AuditError::from("Cannot get value placement in change action, because it has different inner action type"));
    //         },
    //         InnerChangeAction::MODIFY(m) => {
    //             Ok(m.placement.clone())
    //         }
    //     }
    // }

    // pub(crate) fn get_payloads(&self) -> Result<&Vec<Arc<String>>, AuditError> {
    //     match &self.r#type {
    //         InnerChangeAction::ADD(_) => {
    //             return Err(AuditError::from("Cannot get payloads (values) in change action, because it has different inner action type"));
    //         },
    //         InnerChangeAction::MODIFY(m) => {
    //             Ok(&m.values)
    //         }
    //     }
    // }

    pub(crate) fn check_up(&mut self, watch_id_ref: &HashMap<String, usize>) -> Result<(), AuditError> {
        for change in self.changes.iter_mut() {
            match change {
                InnerChangeAction::ADD(adds) => {
                    for add in adds {
                        match add {
                            ChangeAdd::HEADER(h) => {
                                let _ = h.http_header()?;
                            }
                        }
                    }
                },
                InnerChangeAction::MODIFY(modify) => {
                    let splitted_watch_id: Vec<&str> = modify.watch_id.split(".").collect();
                    if splitted_watch_id.len() > 2 {
                        let err_mes = format!("Wwrong format of .watch_id, it should have 0 or 1 '.' (dot) delimiter: {}", &modify.watch_id);
                        return Err(AuditError::from_str(&err_mes).unwrap());
                    }

                    let parsed_watch_id: WatchId = match splitted_watch_id.len() {
                        2 => {
                            // if group name is a number - throw error
                            let group_contains_digits = splitted_watch_id[1]
                                .chars()
                                .any(|c| { c.is_digit(10) });

                            if group_contains_digits {
                                let err_mes = format!("Groups names in watch actions must not contain digits: {}", &modify.watch_id);
                                return Err(AuditError::from_str(&err_mes).unwrap());
                            }

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
                            let err_mes = format!("Wwrong format of .watch_id, it should have 0 or 1 '.' (dot) delimiter and must not be empty: {}", &modify.watch_id);
                            return Err(AuditError::from_str(&err_mes).unwrap());
                        }
                    };

                    modify.watch_id_cache = Some(parsed_watch_id);
                }
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
        let mut steps_results: Vec<ChangeStepResult> = Vec::with_capacity(self.changes.len());

        for (step, change) in self.changes.iter().enumerate() {
            match change {
                InnerChangeAction::ADD(_) => {
                    debug!("ChangeAction - step {} (ADD)", step);
                    steps_results.push(ChangeStepResult::ADD)
                },
                InnerChangeAction::MODIFY(modify) => {
                    let wid = modify.watch_id_cache.as_ref().unwrap();

                    debug!("ChangeAction - step {} (MODIFY) - watch_id '{}' resolved into index {}", step, &modify.watch_id, wid.id);

                    if wid.id >= watch_results.len() {
                        let err_str = format!("action has watch_id '{}' which is resolved into watch-list element with index {} that is greater than the list size - {}", &modify.watch_id, wid.id, watch_results.len());
                        return Err(AuditError(err_str));
                    }

                    let group_name = match wid.group_name.as_ref() {
                        Some(gn) => { gn },
                        None => { "0" }
                    };

                    debug!("ChangeAction - will change capture group from WatchAction {} with name '{}'", wid.id, group_name);

                    let single_watch_result = &watch_results[wid.id];

                    match single_watch_result.get(group_name) {
                        Some(f) => {
                            debug!("ChangeAction - step {} (MODIFY) - found wanted capture", step);
                            let step_result = ChangeStepResult::MODIFY(f.clone());
                            steps_results.push(step_result);
                        },
                        None => {
                            debug!("ChangeAction - step {} (MODIFY) - didn't find wanted capture", step);
                            steps_results.push(ChangeStepResult::NONE);
                        }
                    }
                }
            }
        }

        let all_steps_results = AllChangeStepsResults(steps_results);
        ctxt.add_change_result(all_steps_results);

        Ok(())
    }
}
