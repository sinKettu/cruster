use bstr::ByteSlice;
use http::HeaderMap;
use std::{ffi::CString, collections::HashMap};
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
    let mut tmp_storage: HashMap<&str, SpannedString<Style>> = HashMap::default();
    for (k, v) in headers.iter() {
        let k_str = k.as_str();
        let hval = if let Ok(hval) = v.to_str() {
            StyledString::from(hval)
        }
        else {
            let spanned_hval = StyledString::from(v.as_bytes().to_str_lossy());
            spanned_hval
        };

        let key_found = tmp_storage.get_mut(k_str);
        match key_found {
            Some(val) => {
                // CRUTCH
                // TODO: read RFC, determine the better way
                if k_str == "cookie" {
                    val.append("; ");
                }
                else {
                    val.append(",");
                }
                
                val.append(hval);
            },
            None => {
                tmp_storage.insert(k_str, hval);
            }
        }
    }

    let mut result: SpannedString<Style> = SpannedString::default();
    for (k, v) in tmp_storage {
        result.append(StyledString::styled(k, BaseColor::Blue.dark()));
        result.append(": ");
        result.append(v);
        result.append("\r\n");
    }

    result.append("\r\n");
    return result;
}

pub(super) fn request_wrapper_to_spanned(req: &HyperRequestWrapper) -> SpannedString<Style> {
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
            if body_str.len() > 4000 {
                let tmp = &body_str[..4000];
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
            if body_str.len() > 4000 {
                let tmp = &body_str[..4000];
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