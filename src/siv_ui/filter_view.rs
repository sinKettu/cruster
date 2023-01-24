use regex::Regex;
use cursive::{
    Cursive,
    views::{
        TextView,
        EditView,
        Dialog,
        StackView,
        LinearLayout,
        OnEventView,
        ThemedView
    },
    utils::markup::StyledString,
    theme::{
        Style,
        BaseColor,
        Effect,
        BorderStyle,
        Palette
    },
    view::{
        Nameable,
        Resizable
    },
    event,
    With
};

use super::{sivuserdata::SivUserData, ProxyDataForTable, http_table::HTTPTable};
use crate::http_storage::RequestResponsePair;

pub(super) fn draw_filter(siv: &mut Cursive) {
    let ud: &mut SivUserData = siv.user_data().unwrap();
    let prelude = StyledString::styled(
        " ~ ",
        Style::from(BaseColor::Yellow.light()).combine(Effect::Bold)
    );
    let txt = TextView::new(prelude);

    let editable = EditView::new()
        .on_submit(|s: &mut Cursive, txt: &str| { apply(s, txt); })
        .content(ud.filter_content.clone())
        .with_name("filter-content")
        .full_width();

    let theme = cursive::theme::Theme {
        shadow: false,
        borders: BorderStyle::Simple,
        palette: Palette::default().with(|palette| {
            use cursive::theme::BaseColor::*;
            use cursive::theme::PaletteColor::*;

            palette[View] = Red.light();
            palette[Secondary] = BaseColor::Black.dark();
        }),
    };

    let with_theme = ThemedView::new(theme, editable);
    let layout = LinearLayout::horizontal()
        .child(txt)
        .child(with_theme)
        // .min_height(3)
        .full_width();

    let dialog = Dialog::around(layout).title(" Filter Regex ");
    let with_events = OnEventView::new(dialog)
        .on_event('F', |_s: &mut Cursive| {})
        .on_event(event::Key::Esc, |s: &mut Cursive| { hide_filter(s, None) });
        // .on_event(event::Key::Enter, |s: &mut Cursive| { apply(s) });

    ud.status.set_message("Press <Enter> to apply or <Esc> to go back");
    siv.call_on_name("views-stack", |sv: &mut StackView| { sv.add_layer(with_events) });
    siv.focus_name("filter-content").unwrap();
}

fn apply(siv: &mut Cursive, content: &str) {
    if content.is_empty() {
        super::fill_table_using_scope(siv);
        hide_filter(siv, Some(content));

        return;
    }

    let possible_re = Regex::new(content);
    match possible_re {
        Ok(re) => {
            let table_name = siv.with_user_data(|ud: &mut SivUserData| { ud.active_http_table_name } ).unwrap();
            let table_items = siv.call_on_name(table_name, |table: &mut HTTPTable| {
                // TODO: ensure that popping one is the needed
                table.take_items()
            });
            drop(table_items);

            let ud: &mut SivUserData = siv.user_data().unwrap();
            ud.table_id_ref.clear();

            let mut items: Vec<ProxyDataForTable> = Vec::with_capacity(ud.http_storage.len());
            for pair in ud.http_storage.into_iter() {
                let req = pair.request.as_ref().unwrap();
                let in_scope = ud.is_uri_in_socpe(&req.uri);
                if ! in_scope {
                    continue;
                }

                if is_pair_match_filter(pair, &re) {
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
            }

            siv.call_on_name(table_name, move |table: &mut HTTPTable| { table.set_items(items); });
            hide_filter(siv, Some(content));
        },
        Err(e) => {
            let ud: &mut SivUserData = siv.user_data().unwrap();
            ud.push_error(e.into());

            hide_filter(siv, Some(content));
        }
    }
}

fn hide_filter(siv: &mut Cursive, content: Option<&str>) {
    let ud: &mut SivUserData = siv.user_data().unwrap();
    ud.status.clear_message();

    if let Some(content) = content {
        ud.filter_content = content.to_string();
    }

    siv.call_on_name("views-stack", |sv: &mut StackView| { sv.pop_layer() });
}

pub(super) fn is_pair_match_filter(pair: &RequestResponsePair, re: &Regex) -> bool {
    let req = pair.request.as_ref().unwrap();
    let res = pair.response.as_ref();

    return req.serach_with_re(&re) || (res.is_some() && res.unwrap().serach_with_re(&re));
}