use regex::Regex;
use cursive::{views::TextContent, Cursive};
use tokio::sync::mpsc::Receiver as TokioReceiver;

use super::status_bar;
use crate::{
    config::Config,
    cruster_proxy::request_response::CrusterWrapper,
    utils::CrusterError,
    http_storage::HTTPStorage,
    scope
};
use std::collections::HashMap;

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