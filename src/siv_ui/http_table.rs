use cursive::{Cursive, views::{Dialog, OnEventView, StackView}, view::{Nameable, Resizable}, event::{Key, }, };
use cursive_table_view::TableView;
use super::{BasicColumn, ProxyDataForTable, SivUserData, draw_request_and_response};
use crate::utils::CrusterError;
use std::cmp::Ordering;

pub(super) type HTTPTable = TableView<ProxyDataForTable, BasicColumn>;

pub(super) fn new_table(with_drawing_http_content: bool) -> HTTPTable {
    let table = HTTPTable::new()
        .column(BasicColumn::ID, "ID", |c| c.width(8).ordering(Ordering::Less))
        .column(BasicColumn::Method, "Method", |c| c.width(10))
        .column(BasicColumn::Hostname, "Hostname", |c| {c.width(30)})
        .column(BasicColumn::Path, "Path", |c| {c})
        .column(BasicColumn::StatusCode, "Status", |c| {c.width(16)})
        .column(BasicColumn::ResponseLength, "Length", |c| {c.width(12)});
    
    return if with_drawing_http_content {
        table.on_select(|siv, _, item| { draw_request_and_response(siv, item) })
    }
    else {
        table
    }
}

pub(super) fn make_table_fullscreen(siv: &mut Cursive) {
    let table_items = siv.call_on_name("proxy-table", |table: &mut HTTPTable| {
        // TODO: ensure that popping one is the needed
        table.take_items()
    });

    match table_items {
        Some(items) => {
            let mut table_inst = new_table(false);
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
                ud.active_http_table_name = "fs-proxy-table"
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
    });
}