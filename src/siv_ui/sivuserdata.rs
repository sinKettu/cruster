use std::fs;
use regex::Regex;
use serde_json as json;
use std::collections::HashMap;
use std::io::{Write, BufReader, BufRead};
use cursive::{views::TextContent, Cursive};
use tokio::sync::mpsc::Receiver as TokioReceiver;

use super::repeater;
use super::status_bar;
use super::repeater::RepeaterState;
use super::repeater::RepeaterStateSerializable;
use crate::{
    config::Config,
    cruster_proxy::request_response::CrusterWrapper,
    utils::CrusterError,
    http_storage::HTTPStorage,
    scope
};

pub(super) struct SivUserData {
    pub(super) config: Config,
    pub(super) proxy_receiver: TokioReceiver<(CrusterWrapper, usize)>,
    pub(super) proxy_err_receiver: TokioReceiver<CrusterError>,
    pub(super) http_storage: HTTPStorage,
    pub(super) request_view_content: TextContent,
    pub(super) response_view_content: TextContent,
    pub(super) filter_content: String,
    pub(super) active_http_table_name: &'static str,
    pub(super) errors: Vec<CrusterError>,
    pub(super) status: status_bar::StatusBarContent,
    pub(super) data_storing_started: bool,
    pub(super) include: Option<Vec<Regex>>,
    pub(super) exclude: Option<Vec<Regex>>,
    pub(super) table_id_ref: HashMap<usize, usize>,
    pub(super) repeater_state: Vec<repeater::RepeaterState>,
}

impl SivUserData {
    pub(super) fn receive_data_from_proxy(&mut self) -> Option<(CrusterWrapper, usize)> {
        match self.proxy_receiver.try_recv() {
            Ok(received_data) => {
                Some(received_data)
            },
            Err(e) => {
                self.errors.push(e.into());
                None
            }
        }
    }

    pub(crate) fn push_error(&mut self, err: CrusterError) {
        self.errors.push(err);
        self.update_status();
    }

    pub(super) fn update_status(&mut self) {
        self.status.set_stats(self.errors.len(), self.http_storage.len());
    }

    pub(super) fn is_uri_in_socpe(&self, uri: &str) -> bool {
        match (self.include.as_ref(), self.exclude.as_ref()) {
            (None, None) => {
                return true;
            },
            (Some(included), None) => {
                return scope::fit_included(uri, included.as_slice());
            },
            (None, Some(excluded)) => {
                return scope::fit_excluded(uri, &excluded);
            },
            (Some(inc), Some(exc)) => {
                return scope::fit(uri, &inc, &exc);
            }
        }
    }

    pub(super) fn is_scope_strict(&self) -> bool {
        if let Some(scope) = self.config.scope.as_ref() {
            scope.strict
        }
        else {
            false
        }
    }

    pub(super) fn store_repeater_state(&self, pth: &str) -> Result<(), CrusterError> {
        let mut fout = fs::OpenOptions::new().write(true).create(true).open(pth)?;
        for rs in self.repeater_state.iter() {
            let serializable = RepeaterStateSerializable::from(rs);
            let jsn = json::to_string(&serializable)?;
            let _bytes_written = fout.write(jsn.as_bytes())?;
            let _one_byte_written = fout.write("\n".as_bytes())?;
        }

        Ok(())
    }

    pub(super) fn load_repeater_state(&mut self, pth: &str) -> Result<(), CrusterError> {
        match std::fs::File::open(pth) {
            Ok(fin) => {
                let reader = BufReader::new(fin);
                for read_result in reader.lines() {
                    if let Ok(line) = read_result {
                        if line.is_empty() {
                            continue;
                        }
                        let rs: RepeaterStateSerializable = json::from_str(&line)?;
                        let rss = RepeaterState::try_from(rs)?;
                        self.repeater_state.push(rss);
                    }
                }
            },
            Err(e) => {
                return Err(e.into());
            }
        }

        Ok(())
    }

    pub(super) fn is_http_pair_match_filter(&mut self, id: usize) -> bool {
        let pair = self.http_storage.get_by_id(id);
        if pair.is_none() {
            return false;
        }

        if self.filter_content.is_empty() {
            return true;
        }

        let re = Regex::new(&self.filter_content);
        if re.is_err() {
            return false;
        }

        return super::filter_view::is_pair_match_filter(pair.unwrap(), re.as_ref().unwrap());
    }
}

pub(super) fn make_scope(siv: &mut Cursive) {
    let ud: &mut SivUserData = siv.user_data().unwrap();

    if let Some(scope) = ud.config.scope.as_mut() {
        if let Some(included) = scope.include.as_mut() {
            let compiled = scope::make_re_list(included.as_slice());
            ud.include = Some(compiled);
        }
    }

    if let Some(scope) = ud.config.scope.as_mut() {
        if let Some(excluded) = scope.exclude.as_mut() {
            let compiled = scope::make_re_list(excluded.as_slice());
            ud.exclude = Some(compiled);
        }
    }
}