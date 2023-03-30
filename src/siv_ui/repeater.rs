mod request_executor;
mod repeater_state_implementation;

use cursive::{
    Cursive,
    views::{
        ListView,
        TextView,
        LinearLayout,
        Checkbox,
        EditView,
        OnEventView,
        Dialog,
        SelectView,
        TextArea,
        TextContent
    },
    event,
    view::{
        Resizable,
        Nameable,
        Scrollable
    },
};

// use log::debug;
use http::HeaderMap;
use serde::{Serialize, Deserialize};

use crate::utils::CrusterError;
use super::{sivuserdata::SivUserData, http_table};
use super::{views_stack, req_res_spanned::response_wrapper_to_spanned, sivuserdata::GetCrusterUserData};

#[derive(Serialize, Deserialize, Clone)]
pub(super) struct RepeaterParameters {
    redirects: bool,
    https: bool,
    address: String,
    max_redirects: usize,
}

#[derive(Clone)]
pub(super) struct RepeaterState {
    pub(super) name: String,
    pub(super) request: String,
    pub(super) response: TextContent,
    pub(super) saved_headers: HeaderMap,
    pub(super) redirects_reached: usize,
    pub(super) parameters: RepeaterParameters,
}

#[derive(Serialize, Deserialize)]
pub(super) struct RepeaterStateSerializable {
    name: String,
    request: String,
    response: Option<String>,
    parameters: RepeaterParameters
}

pub(super) fn draw_repeater_select(siv: &mut Cursive) {
    let ud: &mut SivUserData = siv.user_data().unwrap();
    if ud.repeater_state.is_empty() {
        ud.status.set_message("There are no repeaters to select!");
        return;
    }

    let mut initial_list = SelectView::new();
    for (index, instance) in ud.repeater_state.iter().enumerate() {
        let name = instance.name.as_str();
        let label = format!("{}. {}", index, name);
        initial_list.add_item(label, index);
    }

    initial_list.set_on_submit(|s, idx| {
        draw_static_repeater(s, idx.to_owned());
    });

    let name_list = initial_list.with_name("repeaters");
    let with_event = OnEventView::new(name_list)
        .on_event(event::Key::Esc, |s: &mut Cursive| { views_stack::pop_layer(s); });

    let dialog = Dialog::around(with_event)
        .title("Select repeater");

    views_stack::push_layer(siv, dialog);
}

pub(super) fn create_and_draw_repeater(siv: &mut Cursive) {
    let possible_pair_id = http_table::get_selected_id(siv);
    if possible_pair_id.is_none() {
        return;
    }

    let ud: &mut SivUserData = siv.user_data().unwrap();
    let possible_pair = ud.http_storage.get_by_id(possible_pair_id.unwrap());

    if let Some(pair) = possible_pair {
        let idx = ud.repeater_state.len();

        let (content, address, https) = if let Some(req) = pair.request.as_ref() {
            let host = req.get_hostname();
            let https = req.get_scheme().starts_with("https");
            let content = req.to_string();

            (content, host, https)
        }
        else {
            let err = CrusterError::EmptyRequest(
                format!("Could not make repater because of empty request on #{}", pair.index)
            );
            ud.push_error(err);

            return;
        };

        let res_str = if let Some(res) = pair.response.as_ref() {
            TextContent::new(response_wrapper_to_spanned(res))
        }
        else {
            TextContent::new("")
        };

        let repeater_state = RepeaterState {
            name: format!("Repeater #{}", idx),
            request: content,
            response: res_str.clone(),
            saved_headers: HeaderMap::default(),
            redirects_reached: 0,
            parameters: RepeaterParameters {
                redirects: true,
                https,
                address,
                // TODO: make it configurable
                max_redirects: 10
            }
        };

        ud.repeater_state.push(repeater_state);
        draw_static_repeater(siv, idx);
    }
}

fn draw_static_repeater(siv: &mut Cursive, idx: usize) {
    let ud: &mut SivUserData = siv.user_data().unwrap();
    let repeater_state = &mut ud.repeater_state[idx];
    let response_view = TextView::new_with_content(repeater_state.response.clone()).scrollable();
    let request_view = TextView::new(repeater_state.request.as_str())
        .with_name("repeater-request-static")
        .scrollable();

    let request_dialog = Dialog::around(request_view).title(" Request ").full_screen();
    let response_dialog = Dialog::around(response_view).title(" Response ").full_screen();

    let layout = LinearLayout::horizontal()
        .child(request_dialog)
        .child(response_dialog)
        .full_screen();

    let layout_with_quit = OnEventView::new(layout)
        .on_event(event::Key::Esc, |s: &mut Cursive| { views_stack::pop_layer(s); })
        .on_event('p', move |s: &mut Cursive| { draw_repeater_parameters(s, idx.clone()); })
        .on_event(event::Key::Enter, move |s: &mut Cursive| { send_request(s, idx); })
        .on_event('i', move |s: &mut Cursive| { draw_editable_repeater(s, idx); });

    let dialog = Dialog::around(layout_with_quit).title("Repeater").full_screen();
    views_stack::push_fullscreen_layer(siv, dialog);
}

fn draw_editable_repeater(siv: &mut Cursive, idx: usize) {
    let ud: &mut SivUserData = siv.user_data().unwrap();
    let repeater_state = &mut ud.repeater_state[idx];
    
    let request_view = TextArea::new()
        .content(repeater_state.request.as_str())
        .with_name("editable-repeater");

    let response_view = TextView::new_with_content(repeater_state.response.clone());
    let request_dialog = Dialog::around(request_view).title(" Request (Edit) ").full_screen();
    let response_dialog = Dialog::around(response_view).title(" Response ").full_screen();

    let layout = LinearLayout::horizontal()
        .child(request_dialog)
        .child(response_dialog)
        .full_screen();

    let layout_with_quit = OnEventView::new(layout)
        .on_event(event::Key::Esc, move |s: &mut Cursive| { save_and_make_static(s, idx); });

    let dialog = Dialog::around(layout_with_quit).title("Repeater").full_screen();
    views_stack::pop_layer(siv);
    views_stack::push_fullscreen_layer(siv, dialog);
}

fn save_and_make_static(siv: &mut Cursive, idx: usize) {
    let req_content = siv.call_on_name("editable-repeater", |repeater: &mut TextArea| {
        repeater.get_content().to_string()
    }).unwrap();

    let ud: &mut SivUserData = siv.user_data().unwrap();
    let repeater_state = &mut ud.repeater_state[idx];
    repeater_state.request = req_content;

    views_stack::pop_layer(siv);
    draw_static_repeater(siv, idx);
}

fn draw_repeater_parameters(siv: &mut Cursive, idx: usize) {
    let ud: &mut SivUserData = siv.user_data().unwrap();
    let repeater_state = &mut ud.repeater_state[idx];

    let name = EditView::new().content(repeater_state.name.as_str()).with_name("name-edit");
    let host = EditView::new().content(repeater_state.parameters.address.as_str()).with_name("addr-edit");
    let redirect_cb = if repeater_state.parameters.redirects {
        Checkbox::new().checked().with_name("redirect-cb")
    }
    else {
        Checkbox::new().with_name("redirect-cb")
    };

    let https_cb = if repeater_state.parameters.https {
        Checkbox::new().checked().with_name("https-cb")
    }
    else {
        Checkbox::new().with_name("https-cb")
    };

    let list = ListView::new()
        .delimiter()
        .child("Name:", name)
        .child("Host:", host)
        .child("Use HTTPS:", https_cb)
        .child("Follow Redirects:", redirect_cb);
    
    let dialog = Dialog::around(list)
        .title("Edit Repeater Parameters")
        .button("Cancel", |s: &mut Cursive| {views_stack::pop_layer(s);})
        .button("Save", move |s: &mut Cursive| { save_parameters(s, idx); views_stack::pop_layer(s) })
        .min_width(40);

    views_stack::push_layer(siv, dialog);
}

fn save_parameters(siv: &mut Cursive, idx: usize) {
    let name = siv.call_on_name("name-edit", |n: &mut EditView| {
        n.get_content()
    }).unwrap();
 
    let redirects = siv.call_on_name("redirect-cb", |r: &mut Checkbox| {
        r.is_checked()
    }).unwrap();

    let https = siv.call_on_name("https-cb", |h: &mut Checkbox| {
        h.is_checked()
    }).unwrap();

    let host = siv.call_on_name("addr-edit", |host: &mut EditView| {
        host.get_content()
    }).unwrap();

    let ud: &mut SivUserData = siv.user_data().unwrap();
    let repeater_state = &mut ud.repeater_state[idx];
    repeater_state.name = name.to_string();
    repeater_state.parameters = RepeaterParameters {
        redirects,
        https,
        address: host.to_string(),
        max_redirects: 10,
    };
}

fn send_request(siv: &mut Cursive, idx: usize) {
    let ud: &mut SivUserData = siv.user_data().unwrap();
    let repeater_state = &mut ud.repeater_state[idx];
    
    match repeater_state.make_reqwest() {
        Ok(request) => {
            repeater_state.redirects_reached = 1;
            repeater_state.response.set_content("");
            repeater_state.saved_headers = request.headers().clone();
            let need_redirect = repeater_state.parameters.redirects;

            ud.status.set_message("Sending...");
            // request_executor::send_hyper_request(siv, request, Instant::now(), idx);
            request_executor::send_request_detached(request, idx, need_redirect, siv.cb_sink().clone());
        },
        Err(err) => {
            ud.status.set_message("Error when trying to repeat request");
            ud.push_error(err);
        }
    }
}

fn handle_repeater_event(siv: &mut Cursive, state_idx: usize, event: request_executor::RepeaterEvent) {
    match event {
        request_executor::RepeaterEvent::Ready(response) => {
            let ud: &mut SivUserData = siv.user_data().unwrap();
            match ud.repeater_state.get_mut(state_idx) {
                Some(state) => {
                    state.response.set_content(response);
                    ud.status.set_message(format!("Repeater #{} is finished", state_idx));
                },
                None => {
                    let err = CrusterError::UndefinedError(
                        format!("Could not find repeater state #{} after processing request finished", state_idx)
                    );
                    ud.push_error(err);
                    ud.status.set_message(format!("Error in repeater #{}", state_idx));
                }
            }
        },
        request_executor::RepeaterEvent::Error(err) => {
            let ud = siv.get_cruster_userdata();
            ud.push_error(err);
            ud.status.set_message(format!("Error in repeater #{}", state_idx));
        },
        request_executor::RepeaterEvent::RequestChanged(request) => {
            let saved_request = request.clone();
            let result = siv.call_on_name("repeater-request-static", |tv: &mut TextView| {
                tv.set_content(request);
            });

            if result.is_none() {
                let ud = siv.get_cruster_userdata();
                match ud.repeater_state.get_mut(state_idx) {
                    Some(state) => {
                        state.request = saved_request;
                    },
                    None => {
                        let err = CrusterError::UndefinedError(
                            format!("Could not find repeater state #{} while processing redirects", state_idx)
                        );
                        ud.push_error(err);
                        ud.status.set_message(format!("Error in repeater #{}", state_idx));
                    }
                }
            }
            else {
                let ud = siv.get_cruster_userdata();
                ud.status.set_message(format!("Repeater #{} is following redirects", state_idx));
            }
        }
    }
}