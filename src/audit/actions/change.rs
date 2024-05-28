use std::{collections::HashMap, str::FromStr};

use http::{header::HeaderName, HeaderMap, HeaderValue};
use log::debug;
use base64;

use crate::{audit::contexts::traits::{WithChangeAction, WithWatchAction}, http_storage::serializable::Header};

use super::*;

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub(crate) struct ChangeModify {
    placement: ChangeValuePlacement,
    values: Vec<Arc<String>>,
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
                unreachable!("Found unknown header encoding '{}', but it should be checked!", &self.encoding);
            }
        };

        return Ok((name, value));
    }
}

impl RuleChangeAction {
    pub(crate) fn get_inner_action(&self) -> &InnerChangeAction {
        &self.r#type
    }

    pub(crate) fn get_placement(&self) -> Result<ChangeValuePlacement, AuditError> {
        match &self.r#type {
            InnerChangeAction::ADD(_) => {
                return Err(AuditError::from("Cannot get value placement in change action, because it has different inner action type"));
            },
            InnerChangeAction::MODIFY(m) => {
                Ok(m.placement.clone())
            }
        }
    }

    pub(crate) fn get_payloads(&self) -> Result<&Vec<Arc<String>>, AuditError> {
        match &self.r#type {
            InnerChangeAction::ADD(_) => {
                return Err(AuditError::from("Cannot get payloads (values) in change action, because it has different inner action type"));
            },
            InnerChangeAction::MODIFY(m) => {
                Ok(&m.values)
            }
        }
    }

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

        if let InnerChangeAction::ADD(add_actions) = &self.r#type {
            for add_action in add_actions {
                let ChangeAdd::HEADER(h) = add_action;
                let _ = h.http_header()?;
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
