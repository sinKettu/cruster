use http::HeaderMap;
use bstr::ByteSlice;
use std::{ffi::CString, borrow::Cow};
use cursive::{utils::{span::SpannedString, markup::StyledString}, theme::{Style, BaseColor, Effect}};

use crate::cruster_proxy::request_response::{HyperRequestWrapper, HyperResponseWrapper};

fn query_to_spanned(query_str: &str) -> SpannedString<Style> {
    let mut result = SpannedString::from(SpannedString::styled("?", BaseColor::White.dark()));

    let query_without_question = &query_str[1..];
    let split_by_amp: Vec<&str> = query_without_question.split("&").collect();

    for (idx, kv) in split_by_amp.iter().enumerate() {
        let split_by_equal: Vec<&str> = kv.splitn(2, "=").collect();
        if split_by_equal.len() == 1 {
            result.append(
                StyledString::styled(
                    split_by_equal[0],
                    Style::from(BaseColor::Blue.light())
                )
            );
            result.append("=");
        }
        else {
            result.append(
                StyledString::styled(
                    split_by_equal[0],
                    Style::from(BaseColor::Blue.light())
                )
            );
            result.append("=");
            result.append(
                StyledString::styled(
                    split_by_equal[1],
                    BaseColor::Green.light()
                )
            );
        }

        if idx + 1 < split_by_amp.len() {
            result.append(SpannedString::styled("&", BaseColor::White.dark()));
        }
    }

    return result;
}

fn header_map_to_spanned(headers: &HeaderMap) -> SpannedString<Style> {
    let mut result: SpannedString<Style> = SpannedString::default();

    let mut keys_list: Vec<String> = headers
        .keys()
        .into_iter()
        .map(|key| {
            key.to_string()
        })
        .collect();

    keys_list.sort();

    for key in keys_list.iter() {
        let k_str = key.as_str();
        // TODO: handle set-cookie separately
        let v_iter = headers
            .get_all(key)
            .iter()
            .map(|val| {
                val.as_bytes().to_str_lossy()
            })
            .collect::<Vec<Cow<str>>>()
            .join("; ");

        let hval = StyledString::from(v_iter);
        result.append(StyledString::styled(k_str, BaseColor::Blue.dark()));
        result.append(": ");
        result.append(hval);
        result.append("\r\n");
    }

    result.append("\r\n");
    return result;
}

fn request_wrapper_to_spanned_with_limit(req: &HyperRequestWrapper, limit: usize) -> SpannedString<Style> {
    let mut first_line = SpannedString::default();
    let method = SpannedString::styled(&req.method, Style::from(BaseColor::White.light()).combine(Effect::Bold));
    first_line.append(method);
    first_line.append(" ");

    match req.get_request_path_without_query() {
        Ok(path) => {
            let spanned_path = SpannedString::styled(path, BaseColor::Yellow.light());
            first_line.append(spanned_path);

            match req.get_query() {
                Some(query) => {
                    let spanned_query = query_to_spanned(&query);
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

    let headers_content = header_map_to_spanned(&req.headers);
    first_line.append(headers_content);

    let body_str = req.body.to_str_lossy();
    match CString::new(body_str.as_bytes()) {
        Ok(_) => {
            if limit > 0 && body_str.len() > limit {
                let tmp = &body_str[..limit];
                first_line.append("\r\n");
                first_line.append(tmp);

                let tmp = StyledString::styled("--- BODY IS TOO LARGE TO SHOW ---", BaseColor::White.dark());
                first_line.append(tmp);
            }
            else {
                first_line.append(body_str);
            }
        },
        Err(_) => {
            let tmp = StyledString::styled("--- INCOMPATIBLE SET OF BYTES ---", BaseColor::White.dark());
            first_line.append(tmp);
        }
    }

    return first_line;
}

pub(super) fn request_wrapper_to_spanned(req: &HyperRequestWrapper) -> SpannedString<Style> {
    request_wrapper_to_spanned_with_limit(req, 4000)
}

pub(super) fn request_wrapper_to_spanned_full(req: &HyperRequestWrapper) -> SpannedString<Style> {
    request_wrapper_to_spanned_with_limit(req, 0)
}

fn response_to_spanned_with_length_limit(res: &HyperResponseWrapper, limit: usize) -> SpannedString<Style> {
    let mut first_line = SpannedString::default();
    let status = SpannedString::styled(&res.status, BaseColor::Yellow.light());

    first_line.append(&res.version);
    first_line.append(" ");
    first_line.append(status);
    first_line.append("\r\n");

    let headers_content = header_map_to_spanned(&res.headers);
    first_line.append(headers_content);

    let body_str = res.body.to_str_lossy();
    match CString::new(body_str.as_bytes()) {
        Ok(_) => {
            if limit > 0 && body_str.len() > limit {
                let tmp = &body_str[..limit];
                first_line.append("\r\n");
                first_line.append(tmp);

                let tmp = StyledString::styled("--- BODY IS TOO LARGE TO SHOW ---", BaseColor::White.dark());
                first_line.append(tmp);
            }
            else {
                first_line.append(body_str);
            }
        },
        Err(_) => {
            let tmp = StyledString::styled("--- INCOMPATIBLE SET OF BYTES ---", BaseColor::White.dark());
            first_line.append(tmp);
        }
    }

    return first_line;
}

pub(super) fn response_wrapper_to_spanned(res: &HyperResponseWrapper) -> SpannedString<Style> {
    return response_to_spanned_with_length_limit(res, 4000);
}

pub(super) fn response_to_spanned_full(res: &HyperResponseWrapper) -> SpannedString<Style> {
    return response_to_spanned_with_length_limit(res, 0);
}
