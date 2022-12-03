pub(super) mod error_view;
mod help_view;
mod http_table;
mod req_res_spanned;
mod status_bar;
mod sivuserdata;

// use log::debug;
use cursive::{Cursive, };
use cursive::{traits::*, };
use cursive::{CursiveExt, };
use cursive::utils::markup::StyledString;
use cursive_table_view::{TableView, TableViewItem};
use cursive::theme::{BaseColor, BorderStyle, Palette, };
use cursive::views::{Dialog, LinearLayout, TextContent, TextView, StackView, };

use std::rc::Rc;
use std::cmp::Ordering;
use tokio::sync::mpsc::Receiver;

use crate::config::Config;
use sivuserdata::SivUserData;
use crate::utils::CrusterError;
use status_bar::StatusBarContent;
use std::thread::{self, JoinHandle};
use crate::http_storage::HTTPStorage;
use crate::cruster_proxy::request_response::CrusterWrapper;
use log::debug;
use crate::siv_ui::http_table::HTTPTable;

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
    pub(crate) response_length: usize,
}

impl TableViewItem<BasicColumn> for ProxyDataForTable {
    fn to_column(&self, column: BasicColumn) -> String {
        match column {
            BasicColumn::ID => self.id.to_string(),
            BasicColumn::Method => self.method.clone(),
            BasicColumn::Hostname => self.hostname.clone(),
            BasicColumn::Path => self.path.clone(),
            BasicColumn::StatusCode => self.status_code.clone(),
            BasicColumn::ResponseLength => if self.status_code.is_empty() {
                "".to_string()
            }
            else {
                self.response_length.to_string()
            },
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

pub(super) fn bootstrap_ui(mut siv: Cursive, config: Config, rx: Receiver<(CrusterWrapper, usize)>, err_rx: Receiver<CrusterError>) {
    let help_message = Rc::new(help_view::make_help_message());

    siv.add_global_callback('q', |s| s.quit());
    siv.add_global_callback('e', |s| error_view::draw_error_view(s));
    siv.add_global_callback('?', move |s| help_view::draw_help_view(s, &help_message));
    siv.add_global_callback('t', |s| { http_table::make_table_fullscreen(s) });
    siv.add_global_callback('S', |s| { store_proxy_data(s) });

    // siv.set_autorefresh(true);
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

    let status_bar_message = TextContent::new(
        StyledString::styled(" Status bar will be here soon...", BaseColor::Black.light())
    );
    let status_bar_stats = TextContent::new(
        StyledString::styled("Press '?' to get help", BaseColor::Black.light())
    );

    siv.set_user_data(
        SivUserData {
            config,
            proxy_receiver: rx,
            proxy_err_receiver: err_rx,
            http_storage: HTTPStorage::default(),
            request_view_content: request_view_content.clone(),
            response_view_content: response_view_content.clone(),
            active_http_table_name: "proxy-table",
            errors: Vec::new(),
            status: StatusBarContent::new(status_bar_message.clone(), status_bar_stats.clone()),
            data_storing_started: false,
        }
    );

    siv.cb_sink().send(
        Box::new(
            |s: &mut Cursive| { load_data_if_need(s) }
        )
    ).expect("Could not register action to load data from file!");

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
    let base_layout = LinearLayout::vertical()
        .child(status_bar::make_status_bar(status_bar_message, status_bar_stats))
        .child(views_stack.with_name("views-stack").full_width());
    
    siv.add_fullscreen_layer(base_layout);

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
        rx.status.set_stats(rx.errors.len(), rx.http_storage.len());
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

fn poll_storing_thread(siv: &mut Cursive, thrd: JoinHandle<Result<(), CrusterError>>) {
    if thrd.is_finished() {
        let ud: &mut SivUserData = siv.user_data().unwrap();
        match thrd.join() {
            Ok(storing_result) => {
                match storing_result {
                    Ok(_) => {
                        ud.status.clear_message();
                    },
                    Err(err) => {
                        ud.push_error(err);
                        ud.status.set_message("Error occured while storing data...");
                    }
                }
            },
            Err(e) => {
                let err = CrusterError::UndefinedError(
                    format!("Thread with process of storing proxy data failed: {:?}", e)
                );
                ud.push_error(err);
                ud.status.set_message("Error occured while storing data...");
            }
        }
        ud.data_storing_started = false;
    }
    else {
        siv.cb_sink().send(Box::new(
            |s: &mut Cursive| { poll_storing_thread(s, thrd) }
        )).expect("FATAL: Cannot set calback on UI after spawning thread to store proxy data!");
    }
}

fn store_proxy_data(siv: &mut Cursive) {
    let ud: &mut SivUserData = siv.user_data().unwrap();
    if ud.config.store.is_none() {
        return;
    }

    if ud.data_storing_started {
        return;
    }

    ud.data_storing_started = true;
    ud.status.set_message("Storing proxy data...");
    let storage_clone = ud.http_storage.clone();
    let path_to_save = ud.config.store.as_ref().unwrap().clone();
    let thrd = thread::spawn(move || { storage_clone.store(&path_to_save, None) });

    siv.cb_sink().send(Box::new(
        |s: &mut Cursive| { poll_storing_thread(s, thrd) }
    )).expect("FATAL: Cannot set calback on UI after spawning thread to store proxy data!");
}

fn load_data_if_need(siv: &mut Cursive) {
    let ud: &mut SivUserData = siv.user_data().unwrap();
    debug!("{:?}", &ud.config.load);
    if let None = &ud.config.load {
        return;
    }

    let load_path = ud.config.load.as_ref().unwrap();
    let result = ud.http_storage.load(load_path);

    if let Err(e) = result {
        ud.push_error(e);
    }

    let mut items: Vec<ProxyDataForTable> = Vec::with_capacity(ud.http_storage.len() * 2);
    for pair in ud.http_storage.into_iter() {
        let req = pair.request.as_ref().unwrap();
        let mut table_record = ProxyDataForTable {
            id: pair.index,
            method: req.method.clone(),
            hostname: req.get_hostname(),
            path: req.get_request_path(),
            status_code: "".to_string(),
            response_length: 0,
        };

        if let Some(res) = pair.response.as_ref() {
            table_record.status_code = res.status.clone();
            table_record.response_length = res.get_length();
        }

        items.push(table_record);
    }

    let table_name = ud.active_http_table_name;
    siv.call_on_name(table_name, move |t: &mut HTTPTable| {
        t.set_items(items);
    });
}
