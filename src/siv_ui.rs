mod repeater;
mod help_view;
mod quit_popup;
mod http_table;
mod status_bar;
mod views_stack;
mod sivuserdata;
mod filter_view;
mod req_res_spanned;
pub(super) mod error_view;

use cursive::{Cursive, };
use cursive::{traits::*, };
use cursive::{CursiveExt, };
use cursive_table_view::TableViewItem;
use cursive::utils::markup::StyledString;
use cursive::theme::{BaseColor, BorderStyle, Palette, };
use cursive::views::{Dialog, LinearLayout, TextContent, TextView, StackView, };

use log::debug;
use std::rc::Rc;
use std::cmp::Ordering;
use std::collections::HashMap;
use tokio::sync::mpsc::Receiver;
// use std::thread::{self, JoinHandle, sleep};

use std::time::Instant;
use crate::config::Config;
use sivuserdata::{SivUserData};
use crate::utils::CrusterError;
use status_bar::StatusBarContent;
use crate::http_storage::HTTPStorage;
use crate::siv_ui::http_table::HTTPTable;
use crate::cruster_proxy::request_response::CrusterWrapper;

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

    siv.add_global_callback('q', |s| quit_popup::draw_popup(s));
    siv.add_global_callback('e', |s| error_view::draw_error_view(s));
    siv.add_global_callback('?', move |s| help_view::draw_help_view(s, &help_message));
    siv.add_global_callback('t', |s| { http_table::make_table_fullscreen(s) });
    siv.add_global_callback('S', |s| { store_cruster_state(s) });
    siv.add_global_callback('F', |s| { filter_view::draw_filter(s) });
    siv.add_global_callback('r', |s| { repeater::draw_repeater_select(s) });
    siv.add_global_callback('R', |s| { repeater::create_and_draw_repeater(s) });

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
            palette[Secondary] = Green.light();
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
            filter_content: "".to_string(),
            active_http_table_name: "proxy-table",
            errors: Vec::new(),
            status: StatusBarContent::new(status_bar_message.clone(), status_bar_stats.clone()),
            data_storing_started: false,
            include: None,
            exclude: None,
            table_id_ref: HashMap::default(),
            repeater_state: vec![],
        }
    );

    siv.cb_sink().send(
        Box::new(
            |s: &mut Cursive| { sivuserdata::make_scope(s) }
        )
    ).expect("Could not register action to load data from file!");

    siv.cb_sink().send(
        Box::new(
            |s: &mut Cursive| { load_cruster_state(s) }
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

    debug!("Starting Cursive loop");
    siv.run();
}

pub(super) fn put_proxy_data_to_storage(siv: &mut Cursive) {
    let mut rx: SivUserData = siv.take_user_data().unwrap();
    siv.screen_mut().call_on_name(rx.active_http_table_name, |table: &mut HTTPTable| {
        let result = rx.receive_data_from_proxy();
        if let Some((request_or_response, hash)) = result {
            match request_or_response {
                CrusterWrapper::Request(req) => {
                    let fit_scope = rx.is_uri_in_socpe(&req.uri);
                    if !rx.is_scope_strict() || fit_scope {
                        let table_record = rx.http_storage.put_request(req, hash);

                        if fit_scope && rx.is_http_pair_match_filter(table_record.id) {
                            let id = table_record.id;
                            table.insert_item(table_record);
                            let last_index = table.borrow_items().len() - 1;
                            rx.table_id_ref.insert(id, last_index);
                        }
                    }
                },
                CrusterWrapper::Response(res) => {
                    let table_id = rx.http_storage.put_response(res, &hash);
                    if let Some(id) = table_id {
                        if let Some(pair) = rx.http_storage.get_by_id(id) {
                            let response = pair.response.as_ref().unwrap();
                            let table_index = rx.table_id_ref.get(&id);

                            if table_index.is_none() {
                                return;
                            }

                            let possible_table_record = table.borrow_item_mut(table_index.unwrap().to_owned());
                            if let Some(table_record) = possible_table_record {
                                table_record.status_code = response.status.clone();
                                table_record.response_length = response.get_length();
                            }
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
        .call_on_name(table_name, |table: &mut HTTPTable| {
            match table.borrow_item(item) {
                Some(data) => {
                    Some(data.id.clone())
                }
                None => {
                    None
                }
            }
    }).unwrap();

    // TODO: make it sane, move to draw_request_response()
    if let None = id_option {
        let err = CrusterError::ProxyTableIndexOutOfRange(format!("Could not get record with index {}.", item));
        let ud: &mut SivUserData = siv.user_data().unwrap();
        ud.push_error(err);
    }

    return id_option;
}

fn draw_request_and_response(siv: &mut Cursive, item: usize) {
    let possible_id = get_pair_id_from_table_record(siv, item);
    let user_data: &mut SivUserData = siv.user_data().unwrap();
    user_data.request_view_content.set_content("");
    user_data.response_view_content.set_content("");

    if let Some(id) = possible_id {
        let pair = user_data.http_storage.get_by_id(id);

        if let Some(pair) = pair {
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
}

/// For now it stores http history + repeater state
fn store_cruster_state(siv: &mut Cursive) {
    let ud: &mut SivUserData = siv.user_data().unwrap();
    if ud.config.store.is_none() {
        ud.push_error(
            CrusterError::StorePathNotFoundError(
                format!("You tried to store data, but did not specify path to store at start.")
            )
        );
        
        return;
    }

    if ud.data_storing_started {
        return;
    }

    ud.data_storing_started = true;
    ud.status.set_message("Storing state...");
    
    debug!("Dir to store: {}", ud.config.store.as_ref().unwrap());
    let path_to_save = ud.config.store.as_ref().unwrap();
    let http_path = format!("{}/http.jsonl", path_to_save);
    let rs_path = format!("{}/repeater.jsonl", path_to_save);

    let http_storing_result = ud.http_storage.store(&http_path, None);
    if let Err(err) = http_storing_result {
        ud.push_error(err);
        ud.status.set_message("Error when storing http data");
    }

    let rs_storing_result = ud.store_repeater_state(&rs_path);
    if let Err(err) = rs_storing_result {
        ud.push_error(err);
        ud.status.set_message("Error when storing repeater state");
    }

    ud.data_storing_started = false;
    ud.status.set_message("Storing completed");
}

fn load_cruster_state(siv: &mut Cursive) {
    let ud: &mut SivUserData = siv.user_data().unwrap();

    if let None = &ud.config.load {
        debug!("Load dir not found");
        return;
    }
    else {
        let load_dir = ud.config.load.as_ref().unwrap();
        debug!("Loading state from \"{}\"", load_dir.to_string());
    }

    let load_path = format!("{}/http.jsonl", ud.config.load.as_ref().unwrap());
    let result = if ud.is_scope_strict() {
        ud.http_storage.load_with_strict_scope(
            &load_path,
            ud.include.as_ref(),
            ud.exclude.as_ref()
        )
    }
    else {
        ud.http_storage.load(&load_path)
    };

    if let Err(e) = result {
        ud.push_error(e);
        ud.status.set_message("Error while loading HTTP data from file");
    }
    
    let load_path = format!("{}/repeater.jsonl", ud.config.load.as_ref().unwrap());
    let result = ud.load_repeater_state(&load_path);

    if let Err(err) = result {
        ud.push_error(err);
        ud.status.set_message("Error while loading repeater state from file");
    }

    fill_table_using_scope(siv);
}

// fn load_data_if_need(siv: &mut Cursive) {
//     let ud: &mut SivUserData = siv.user_data().unwrap();
//     debug!("{:?}", &ud.config.load);
//     if let None = &ud.config.load {
//         return;
//     }

//     let load_path = ud.config.load.as_ref().unwrap();

//     let result = if ud.is_scope_strict() {
//         ud.http_storage.load_with_strict_scope(load_path, ud.include.as_ref(), ud.exclude.as_ref())
//     }
//     else {
//         ud.http_storage.load(load_path)
//     };

//     if let Err(e) = result {
//         ud.push_error(e);
//     }

//     fill_table_using_scope(siv);
// }

fn fill_table_using_scope(siv: &mut Cursive) {
    let a = Instant::now();
    let ud: &mut SivUserData = siv.user_data().unwrap();
    let mut items: Vec<ProxyDataForTable> = Vec::with_capacity(ud.http_storage.len() * 2);
    ud.table_id_ref.clear();

    for pair in ud.http_storage.into_iter() {
        let req = pair.request.as_ref().unwrap();

        let in_scope = ud.is_uri_in_socpe(&req.uri);
        if ! in_scope {
            continue;
        }

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

        let id = table_record.id;
        items.push(table_record);
        ud.table_id_ref.insert(id, items.len() - 1);
    }

    let table_name = ud.active_http_table_name;
    ud.update_status();
    debug!("Took: {} ms", Instant::now().duration_since(a).as_millis());
    siv.call_on_name(table_name, move |t: &mut HTTPTable| {
        t.set_items(items);
    });
}