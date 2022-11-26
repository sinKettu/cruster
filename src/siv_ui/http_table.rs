use cursive::{
    Cursive,
    views::{
        Dialog,
        OnEventView,
        StackView,
        TextView,
        LinearLayout
    },
    view::{
        Nameable,
        Resizable,
        Scrollable
    },
    event::{
        Key,
    },
};
use cursive_table_view::TableView;
use std::cmp::Ordering;

use super::{BasicColumn, ProxyDataForTable, SivUserData, draw_request_and_response};
use crate::utils::CrusterError;

// use log::debug;

pub(super) type HTTPTable = TableView<ProxyDataForTable, BasicColumn>;

pub(super) fn new_table() -> HTTPTable {
    let table: HTTPTable = HTTPTable::new()
        .on_submit(|siv: &mut Cursive, _: usize, __: usize| { draw_fullscreen_request_and_response(siv); })
        .on_select(|siv, _, item| { draw_request_and_response(siv, item) })
        .column(
            BasicColumn::ID,
            "ID",
            |c| c.width(8).ordering(Ordering::Less)
        )
        .column(
            BasicColumn::Method,
            "Method",
            |c| c.width(10)
        )
        .column(
            BasicColumn::Hostname,
            "Hostname",
            |c| {c.width(30)}
        )
        .column(
            BasicColumn::Path,
            "Path",
            |c| {c}
        )
        .column(
            BasicColumn::StatusCode,
            "Status",
            |c| {c.width(16)}
        )
        .column(
            BasicColumn::ResponseLength,
            "Length",
            |c| {c.width(12)}
        );

    return table;
}

pub(super) fn make_table_fullscreen(siv: &mut Cursive) {
    if siv.find_name::<HTTPTable>("fs-proxy-table").is_some() { return; }



    let table_items = siv.call_on_name("proxy-table", |table: &mut HTTPTable| {
        // TODO: ensure that popping one is the needed
        table.take_items()
    });

    match table_items {
        Some(items) => {
            let mut table_inst = new_table();
            table_inst.set_items(items);

            siv.call_on_name("views-stack", |sv: &mut StackView| {
                sv.add_fullscreen_layer(
                    Dialog::around(
                        OnEventView::new(
                            table_inst.with_name("fs-proxy-table").full_screen()
                        )
                            .on_event(Key::Esc, |s| {
                                remove_fullscreen_http_proxy(s)
                            })
                    )
                        .title("Proxy (Fullscreen)")    
                )
            });

            siv.with_user_data(|ud: &mut SivUserData| {
                ud.active_http_table_name = "fs-proxy-table";
                ud.status.set_message("Press 'Esc' to go back");
            });
        },
        None => {
            let ud: &mut SivUserData = siv.user_data().unwrap();
            ud.push_error(
                CrusterError::UndefinedError("Could not take items from table in fullscreen method".to_string())
            );
        }
    }
}

fn remove_fullscreen_http_proxy(siv: &mut Cursive) {
    let table_items = siv.call_on_name("fs-proxy-table", |table: &mut HTTPTable| {
        // TODO: ensure that popping one is the needed
        table.take_items()
    }).unwrap();

    siv.call_on_name("proxy-table", |table: &mut HTTPTable| {
        let _ = table.take_items();
        table.set_items(table_items);
    });

    siv.call_on_name("views-stack", |sv: &mut StackView| {
        sv.pop_layer();
    });

    siv.with_user_data(|ud: &mut SivUserData| {
        ud.active_http_table_name = "proxy-table";
        ud.status.clear_message();
    });
}

fn draw_fullscreen_request_and_response(siv: &mut Cursive) {
    let (request_content, response_content) = siv.with_user_data(|ud: &mut SivUserData| {
        let req_content = ud.request_view_content.clone();
        let res_content = ud.response_view_content.clone();
        (req_content, res_content)
    }).unwrap();

    siv.call_on_name("views-stack", move |sv: &mut StackView| {
        // Does not work without second clone, I do not know why
        let request_view = TextView::new_with_content(request_content.clone())
            .scrollable();
        let response_view = TextView::new_with_content(response_content.clone())
            .full_screen()
            .scrollable();

        let layout = LinearLayout::horizontal()
            .child(Dialog::around(request_view).title("Request").with_name("request-fs"))
            .child(Dialog::around(response_view).title("Response").with_name("response-fs"))
            .full_screen();

        let layout_with_event = OnEventView::new(layout)
            .on_event(Key::Esc, |s: &mut Cursive| {
                s.call_on_name("views-stack", |sv: &mut StackView| { sv.pop_layer(); });
            })
            .on_event(Key::Left, |s: &mut Cursive| {
                s.focus_name("request-fs").unwrap();
            })
            .on_event(Key::Right, |s: &mut Cursive| {
                s.focus_name("response-fs").unwrap();
            });
        
        sv.add_layer(layout_with_event);
            
    });    
}
