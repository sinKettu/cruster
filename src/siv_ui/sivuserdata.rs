use cursive::views::TextContent;
use tokio::sync::mpsc::Receiver as TokioReceiver;

use super::status_bar;
use crate::{
    config::Config,
    cruster_proxy::request_response::CrusterWrapper,
    utils::CrusterError,
    http_storage::HTTPStorage
};

pub(super) struct SivUserData {
    pub(super) config: Config,
    pub(super) proxy_receiver: TokioReceiver<(CrusterWrapper, usize)>,
    pub(super) proxy_err_receiver: TokioReceiver<CrusterError>,
    pub(super) http_storage: HTTPStorage,
    pub(super) request_view_content: TextContent,
    pub(super) response_view_content: TextContent,
    pub(super) active_http_table_name: &'static str,
    pub(super) errors: Vec<CrusterError>,
    pub(super) status: status_bar::StatusBarContent,
    pub(super) data_storing_started: bool,
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
}
