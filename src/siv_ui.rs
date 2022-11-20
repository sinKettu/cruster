use std::cmp::Ordering;
use cursive::traits::*;
use cursive::{CursiveExt, Vec2};
use cursive::{Cursive, Printer};
use std::collections::VecDeque;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use cursive::event::Event;
use cursive::theme::{BaseColor, BorderStyle, Color, Palette, PaletteColor};
use cursive::view::scroll::required_size;
use cursive::views::{Dialog, LinearLayout, TextView};
use cursive_table_view::{TableView, TableViewItem};

use tokio::sync::mpsc::{Sender, Receiver, channel};

use crate::cruster_proxy::request_response::{CrusterWrapper, HyperRequestWrapper};
use crate::utils::CrusterError;
use crate::http_storage;
use crate::http_storage::HTTPStorage;

struct SivUserData {
    proxy_receiver: Receiver<(CrusterWrapper, usize)>,
    proxy_err_receiver: Receiver<CrusterError>,
    http_storage: HTTPStorage,
}

impl SivUserData {
    pub(super) fn receive_data_from_proxy(&mut self) -> Option<(CrusterWrapper, usize)> {
        match self.proxy_receiver.try_recv() {
            Ok(received_data) => {
                Some(received_data)
            },
            Err(e) => {
                // TODO: Log error
                None
            }
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum BasicColumn {
    ID,
    Method,
    Hostname,
    Path
}

// Define the item type
#[derive(Clone, Debug)]
pub(crate) struct ProxyDataForTable {
    pub(crate) id: usize,
    pub(crate) method: String,
    pub(crate) hostname: String,
    pub(crate) path: String,
}

impl ProxyDataForTable {
    fn from_request(request: HyperRequestWrapper, id: usize) -> ProxyDataForTable {
        ProxyDataForTable {
            id,
            hostname: request.get_host(),
            path: request.get_request_path(),
            method: request.method.clone()
        }
    }
}

impl TableViewItem<BasicColumn> for ProxyDataForTable {
    fn to_column(&self, column: BasicColumn) -> String {
        match column {
            BasicColumn::ID => self.id.to_string(),
            BasicColumn::Method => self.method.clone(),
            BasicColumn::Hostname => self.hostname.clone(),
            BasicColumn::Path => self.path.clone()
        }
    }

    fn cmp(&self, other: &Self, column: BasicColumn) -> Ordering where Self: Sized {
        match column {
            BasicColumn::ID => self.id.cmp(&other.id),
            BasicColumn::Method => self.method.cmp(&other.method),
            BasicColumn::Hostname => self.hostname.cmp(&other.hostname),
            BasicColumn::Path => self.path.cmp(&other.path)
        }
    }
}


pub(super) fn bootstrap_ui(mut siv: Cursive, rx: Receiver<(CrusterWrapper, usize)>, err_rx: Receiver<CrusterError>) {
    siv.add_global_callback('q', |s| s.quit());
    siv.set_theme(cursive::theme::Theme {
        shadow: false,
        borders: BorderStyle::Simple,
        palette: Palette::default().with(|palette| {
            use cursive::theme::BaseColor::*;
            use cursive::theme::Color::TerminalDefault;
            use cursive::theme::PaletteColor::*;

            palette[Background] = TerminalDefault;
            palette[View] = TerminalDefault;
            palette[Primary] = White.light();
            palette[TitlePrimary] = Green.light();
            palette[Secondary] = Red.light();
            palette[Highlight] = White.light();
            palette[HighlightText] = BaseColor::Black.dark();
        }),
    });

    siv.set_user_data(
        SivUserData {
            proxy_receiver: rx,
            proxy_err_receiver: err_rx,
            http_storage: HTTPStorage::default(),
        }
    );

    let mut main_table = TableView::<ProxyDataForTable, BasicColumn>::new()
        .on_select(
            |siv, _, item| {
                let id = get_pair_id_from_table_record(siv, item);
                if let Some(index) = id {
                    let user_data: &mut SivUserData = siv.user_data().unwrap();
                    let pair = user_data.http_storage.get(index);
                }
             }
        )
        .column(BasicColumn::ID, "ID", |c| c.width(6).ordering(Ordering::Less))
        .column(BasicColumn::Method, "Method", |c| c.width(10))
        .column(BasicColumn::Hostname, "Hostname", |c| {c})
        .column(BasicColumn::Path, "Path", |c| {c})
        .with_name("proxy-table");

    siv.add_layer(
        LinearLayout::vertical()
            .child(
                Dialog::around(
                    main_table
                        .full_screen()
                )
                    .title("Proxy")
            )
            .child(
                LinearLayout::horizontal()
                    .child(
                        Dialog::around(
                            TextView::new("One")
                                .full_screen()
                        )
                            .title("Request")
                    )
                    .child(Dialog::around(TextView::new("Two").full_screen()).title("Response"))
            )
    );

    siv.run();
}

pub(super) fn put_proxy_data_to_storage(siv: &mut Cursive) {
    // TODO: change move to borrow
    let mut rx: SivUserData = siv.take_user_data().unwrap();
    siv.screen_mut().call_on_name("proxy-table", |table: &mut TableView<ProxyDataForTable, BasicColumn>| {
        let result = rx.receive_data_from_proxy();
        if let Some((request_or_response, hash)) = result {
            match request_or_response {
                CrusterWrapper::Request(req) => {
                    let table_record = rx.http_storage.put_request(req, hash);
                    table.insert_item(table_record);
                },
                CrusterWrapper::Response(res) => {
                    rx.http_storage.put_response(res, &hash);
                }
            }
        }
    });

    siv.set_user_data(rx);
}

fn get_pair_id_from_table_record(siv: &mut Cursive, item: usize) -> Option<usize> {
    siv.screen_mut().call_on_name("proxy-table", |table: &mut TableView<ProxyDataForTable, BasicColumn>| {
        match table.borrow_item(item) {
            Some(data) => {
                Some(data.id.clone())
            }
            None => {
                None
            }
        }
    }).unwrap()
}
