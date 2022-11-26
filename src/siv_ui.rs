pub(super) mod error_view;
mod help_view;
mod http_table;
mod req_res_spanned;

use std::cmp::Ordering;
use cursive::{traits::*, };
use cursive::{CursiveExt, };
use cursive::{Cursive, };

use cursive::theme::{BaseColor, BorderStyle, Palette, };
use cursive::views::{Dialog, LinearLayout, TextContent, TextView, StackView, };
use cursive_table_view::{TableView, TableViewItem};

use tokio::sync::mpsc::{Receiver, };

use crate::cruster_proxy::request_response::{CrusterWrapper, };
use crate::utils::CrusterError;
use crate::http_storage::{HTTPStorage, };
use std::rc::Rc;
// use log::debug;

struct SivUserData {
    proxy_receiver: Receiver<(CrusterWrapper, usize)>,
    proxy_err_receiver: Receiver<CrusterError>,
    http_storage: HTTPStorage,
    request_view_content: TextContent,
    response_view_content: TextContent,
    active_http_table_name: &'static str,
    errors: Vec<CrusterError>,
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

    fn push_error(&mut self, err: CrusterError) {
        self.errors.push(err);
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum BasicColumn {
    ID,
    Method,
    Hostname,
    Path,
    StatusCode,
    ResponseLength,
}

// Define the item type
#[derive(Clone, Debug)]
pub(crate) struct ProxyDataForTable {
    pub(crate) id: usize,
    pub(crate) method: String,
    pub(crate) hostname: String,
    pub(crate) path: String,
    pub(crate) status_code: String,
    pub(crate) response_length: String,
}

impl TableViewItem<BasicColumn> for ProxyDataForTable {
    fn to_column(&self, column: BasicColumn) -> String {
        match column {
            BasicColumn::ID => self.id.to_string(),
            BasicColumn::Method => self.method.clone(),
            BasicColumn::Hostname => self.hostname.clone(),
            BasicColumn::Path => self.path.clone(),
            BasicColumn::StatusCode => self.status_code.clone(),
            BasicColumn::ResponseLength => self.response_length.clone(),
        }
    }

    fn cmp(&self, other: &Self, column: BasicColumn) -> Ordering where Self: Sized {
        match column {
            BasicColumn::ID => self.id.cmp(&other.id),
            BasicColumn::Method => self.method.cmp(&other.method),
            BasicColumn::Hostname => self.hostname.cmp(&other.hostname),
            BasicColumn::Path => self.path.cmp(&other.path),
            BasicColumn::StatusCode => self.status_code.cmp(&other.status_code),
            BasicColumn::ResponseLength => self.response_length.cmp(&other.response_length),
        }
    }
}

pub(super) fn bootstrap_ui(mut siv: Cursive, rx: Receiver<(CrusterWrapper, usize)>, err_rx: Receiver<CrusterError>) {
    let help_message = Rc::new(help_view::make_help_message());

    siv.add_global_callback('q', |s| s.quit());
    siv.add_global_callback('e', |s| error_view::draw_error_view(s));
    siv.add_global_callback('?',  move |s| help_view::draw_help_view(s, &help_message));
    siv.add_global_callback('t', |s| { http_table::make_table_fullscreen(s) });

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

    let request_view_content = TextContent::new("");
    let response_view_content = TextContent::new("");

    siv.set_user_data(
        SivUserData {
            proxy_receiver: rx,
            proxy_err_receiver: err_rx,
            http_storage: HTTPStorage::default(),
            request_view_content: request_view_content.clone(),
            response_view_content: response_view_content.clone(),
            active_http_table_name: "proxy-table",
            errors: Vec::new(),
        }
    );

    let main_table = http_table::new_table().with_name("proxy-table");
    let mut views_stack = StackView::new();

    views_stack.add_fullscreen_layer(
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
                            TextView::new_with_content(request_view_content)
                                .full_screen()
                        ).title("Request")
                    )
                    .child(
                        Dialog::around(
                            TextView::new_with_content(response_view_content)
                                .full_screen()
                        ).title("Response")
                    )
            )
    );
    
    siv.add_fullscreen_layer(views_stack.with_name("views-stack"));

    siv.run();
}

pub(super) fn put_proxy_data_to_storage(siv: &mut Cursive) {
    let mut rx: SivUserData = siv.take_user_data().unwrap();
    siv.screen_mut().call_on_name(rx.active_http_table_name, |table: &mut TableView<ProxyDataForTable, BasicColumn>| {
        let result = rx.receive_data_from_proxy();
        if let Some((request_or_response, hash)) = result {
            match request_or_response {
                CrusterWrapper::Request(req) => {
                    let table_record = rx.http_storage.put_request(req, hash);
                    table.insert_item(table_record);
                },
                CrusterWrapper::Response(res) => {
                    let table_idx = rx.http_storage.put_response(res, &hash);
                    if let Some(idx) = table_idx {
                        let response = rx
                            .http_storage
                            .get(idx)
                            .response
                            .as_ref()
                            .unwrap();
                        
                        let table_record_option = table.borrow_item_mut(idx);
                        if let Some(table_record) = table_record_option {
                            table_record.status_code = response.status.clone();
                            table_record.response_length = response.get_length();
                        }
                    }
                }
            }
        }
    });

    siv.set_user_data(rx);
}

fn get_pair_id_from_table_record(siv: &mut Cursive, item: usize) -> Option<usize> {
    let table_name = siv.with_user_data(|ud: &mut SivUserData| { ud.active_http_table_name } ).unwrap();
    let id_option = siv
        .screen_mut()
        .call_on_name(table_name, |table: &mut TableView<ProxyDataForTable, BasicColumn>| {
            match table.borrow_item(item) {
                Some(data) => {
                    Some(data.id.clone())
                }
                None => {
                    None
                }
            }
    }).unwrap();

    if let None = id_option {
        let err = CrusterError::ProxyTableIndexOutOfRange(format!("Could not get record with index {}.", item));
        let ud: &mut SivUserData = siv.user_data().unwrap();
        ud.push_error(err);
    }

    return id_option;
}

fn draw_request_and_response(siv: &mut Cursive, item: usize) {
    let id = get_pair_id_from_table_record(siv, item);
    let user_data: &mut SivUserData = siv.user_data().unwrap();
    user_data.request_view_content.set_content("");
    user_data.response_view_content.set_content("");

    if let Some(index) = id {
        let pair = user_data.http_storage.get(index);

        if let Some(request) = &pair.request {
            let req_spanned = req_res_spanned::request_wrapper_to_spanned(request);
            user_data.request_view_content.set_content(req_spanned);

            if let Some(response) = &pair.response {
                let res_spanned = req_res_spanned::response_wrapper_to_spanned(response);
                user_data.response_view_content.set_content(res_spanned);
            }
        }
        else {
            user_data.push_error(CrusterError::EmptyRequest(format!("Could not draw table record {}, request is empty.", item)));
        }
    }
}
