mod request_executor;

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
use base64;
use regex::Regex;
use hyper::body;
use serde::{Serialize, Deserialize};
use http::{HeaderValue, header::HeaderName, HeaderMap};
use std::{str::FromStr, thread::JoinHandle, time::Instant};

use super::views_stack;
use crate::utils::CrusterError;
use super::{sivuserdata::SivUserData, http_table};
use bstr::ByteSlice;

type RepeaterRequestHandler = JoinHandle<Result<hyper::Response<hyper::body::Body>, hyper::Error>>;

#[derive(Serialize, Deserialize, Clone)]
pub(super) struct RepeaterParameters {
    redirects: bool,
    https: bool,
    address: String,
    max_redirects: usize,
}

#[derive(Clone)]
pub(super) struct RepeaterState {
    name: String,
    request: String,
    response: TextContent,
    saved_headers: HeaderMap,
    redirects_reached: usize,
    parameters: RepeaterParameters,
}

#[derive(Serialize, Deserialize)]
pub(super) struct RepeaterStateSerializable {
    name: String,
    request: String,
    response: Option<String>,
    parameters: RepeaterParameters
}

impl From<&RepeaterState> for RepeaterStateSerializable {
    fn from(rs: &RepeaterState) -> Self {
        let rsp_content = rs.response.get_content();
        let rsp_raw = rsp_content.source();
        let rsp = if rsp_raw.is_empty() {
            None
        }
        else {
            Some(base64::encode(rsp_raw.as_bytes()))
        };

        RepeaterStateSerializable {
            name: rs.name.clone(),
            request: base64::encode(rs.request.as_bytes()),
            response: rsp,
            parameters: rs.parameters.clone()
        }
    }
}

impl From<RepeaterStateSerializable> for RepeaterState {
    fn from(rss: RepeaterStateSerializable) -> Self {
        let response = match rss.response.as_ref() {
            Some(rsp) => {
                let rsp_raw = base64::decode(rsp).unwrap();
                TextContent::new(rsp_raw.to_str().unwrap())
            },
            None => {
                TextContent::new("")
            }
        };
        
        // TODO: handle errors. Possibly it should be rewritten as TryInto
        let req_raw = base64::decode(rss.request).unwrap();
        let req_str = String::from_utf8(req_raw).unwrap();

        RepeaterState {
            name: rss.name,
            request: req_str,
            response,
            saved_headers: HeaderMap::default(),
            redirects_reached: 0,
            parameters: rss.parameters
        }
    }
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
            TextContent::new(res.to_string())
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
    // TODO: all regexes from this method can be reviewed
    // May be will be rewritten with special parser module
    let ud: &mut SivUserData = siv.user_data().unwrap();
    let repeater_state = &mut ud.repeater_state[idx];
    let req_fl = repeater_state.request.splitn(2, "\r\n").collect::<Vec<&str>>()[0];
    let fl_regex = Regex::new(r"^(?P<method>[\w]+) (?P<path>.*) (?P<version>HTTP/(\d+\.)?\d+)$").unwrap();

    let (method, uri, version) =  match fl_regex.captures(req_fl) {
        Some(captures) => {
            let method = &captures["method"];
            let path = &captures["path"];
            let version = &captures["version"];

            let scheme = if repeater_state.parameters.https { "https" } else { "http" };
            let hostname = repeater_state.parameters.address.as_str();
            let uri = format!("{}://{}{}", scheme, hostname, path);

            (method.to_string(), uri, version.to_string())
        },
        None => {
            let err_str = format!("Could parse first line of request in repeater: {}", req_fl.clone());
            ud.push_error(CrusterError::RegexError(err_str));

            return;
        }
    };

    let version = if version == "HTTP/0.9" {
        http::version::Version::HTTP_09
    }
    else if version == "HTTP/1.0" {
        http::version::Version::HTTP_10
    }
    else if version == "HTTP/1.1" {
        http::version::Version::HTTP_11
    }
    else if version == "HTTP/2" || version == "HTTP/2.0" {
        http::version::Version::HTTP_2
    }
    else if version == "HTTP/3" || version == "HTTP/3.0" {
        http::version::Version::HTTP_3
    }
    else {
        let err_str = format!("Unknown HTTP version of request in repeater: {}", &version);
        ud.push_error(CrusterError::UndefinedError(err_str));

        return;
    };

    let mut request_builder = hyper::Request::builder()
        .method(method.as_str())
        .uri(uri)
        .version(version);

    let header_re = Regex::new(r"^(?P<name>[\d\w_\-]+): (?P<val>.*)$").unwrap();
    let mut body = String::with_capacity(repeater_state.request.len());
    let mut the_following_is_body = false;
    for line in repeater_state.request.split("\r\n").skip(1) {
        if line.is_empty() {
            the_following_is_body = true;
            continue;
        }

        if the_following_is_body {
            body.push_str(line);
            body.push_str("\r\n");
            continue;
        }

        match header_re.captures(line) {
            Some(cap) => {
                let str_name = &cap["name"];
                //  TODO handle unwrappings
                let name = HeaderName::from_str(str_name).unwrap();
                // TODO parse something like \x0e in headers
                let str_val = &cap["val"];
                //  TODO handle unwrappings
                let val = HeaderValue::from_str(str_val).unwrap();
                let headers = request_builder.headers_mut().unwrap();

                headers.insert(name, val);
            },
            None => {
                let str_err = CrusterError::RegexError(
                    format!("Could not parse headers in repeater from {}", line)
                );
                ud.push_error(str_err);

                return;
            }
        }
    }

    repeater_state.saved_headers = request_builder.headers_ref().unwrap().clone();
    match request_builder.body(body::Body::from(body)) {
        Ok(request) => {
            repeater_state.redirects_reached = 1;
            repeater_state.response.set_content("");
            // ud.push_error(CrusterError::UndefinedError(uri.clone()));
            ud.status.set_message("Sending...");
            request_executor::send_hyper_request(siv, request, Instant::now(), idx);
        },
        Err(e) => {
            let err = CrusterError::HyperRequestBuildingError(
                format!("Could not parse request: {}", e.to_string())
            );

            ud.push_error(err);
        }
    }
}

