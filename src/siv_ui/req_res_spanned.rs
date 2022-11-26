use crate::cruster_proxy::request_response::{HyperRequestWrapper, HyperResponseWrapper};
use cursive::{utils::{span::SpannedString, markup::StyledString}, theme::{Style, BaseColor, Effect}};
use bstr::ByteSlice;
use std::ffi::CString;


pub(super) fn request_wrapper_to_spanned(req: &HyperRequestWrapper) -> SpannedString<Style> {
    let mut first_line = SpannedString::default();
    let method = SpannedString::styled(&req.method, BaseColor::Blue.light());
    first_line.append(method);
    first_line.append(" ");

    match req.get_request_path_without_query() {
        Ok(path) => {
            let spanned_path = SpannedString::styled(path, BaseColor::Yellow.light());
            first_line.append(spanned_path);

            match req.get_query() {
                Some(query) => {
                    let spanned_query = SpannedString::styled(query, Effect::Bold);
                    first_line.append(spanned_query);
                },
                None => {
                
                }
            }
        },
        Err(_) => {
            let spanned_path = SpannedString::styled(&req.uri, Effect::Bold);
            first_line.append(spanned_path);
        }
    }
    
    first_line.append(" ");
    first_line.append(&req.version);
    first_line.append("\r\n");

    let mut headers_content = SpannedString::default();
    for (k, v) in &req.headers {
        headers_content.append(StyledString::styled(k.as_str(), BaseColor::Blue.dark()));
        headers_content.append(": ");
        headers_content.append(v.to_str().unwrap());
        headers_content.append("\r\n");
    }

    headers_content.append("\r\n");
    first_line.append(headers_content);

    let body_str = req.body.to_str_lossy();
    match CString::new(body_str.as_bytes()) {
        Ok(_) => {
            if body_str.len() > 4000 {
                let tmp = &body_str[..4000];
                first_line.append("\r\n");
                first_line.append(tmp);

                let tmp = StyledString::styled("--- BODY IS TOO LARGE TO SHOW ---", BaseColor::Black.light());
                first_line.append(tmp);
            }
            else {
                first_line.append(body_str);
            }
        },
        Err(_) => {
            let tmp = StyledString::styled("--- INCOMPATIBLE SET OF BYTES ---", BaseColor::Black.light());
            first_line.append(tmp);
        }
    }

    return first_line;
}

pub(super) fn response_wrapper_to_spanned(res: &HyperResponseWrapper) -> SpannedString<Style> {
    let mut first_line = SpannedString::default();
    let status = SpannedString::styled(&res.status, BaseColor::Yellow.light());

    first_line.append(&res.version);
    first_line.append(" ");
    first_line.append(status);
    first_line.append("\r\n");

    let mut headers_content = SpannedString::default();
    for (k, v) in &res.headers {
        headers_content.append(StyledString::styled(k.as_str(), BaseColor::Blue.dark()));
        headers_content.append(": ");
        headers_content.append(v.to_str().unwrap());
        headers_content.append("\r\n");
    }

    headers_content.append("\r\n");
    first_line.append(headers_content);

    let body_str = res.body.to_str_lossy();
    match CString::new(body_str.as_bytes()) {
        Ok(_) => {
            if body_str.len() > 4000 {
                let tmp = &body_str[..4000];
                first_line.append("\r\n");
                first_line.append(tmp);

                let tmp = StyledString::styled("--- BODY IS TOO LARGE TO SHOW ---", BaseColor::Black.light());
                first_line.append(tmp);
            }
            else {
                first_line.append(body_str);
            }
        },
        Err(_) => {
            let tmp = StyledString::styled("--- INCOMPATIBLE SET OF BYTES ---", BaseColor::Black.light());
            first_line.append(tmp);
        }
    }

    return first_line;
}