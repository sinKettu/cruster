use crate::cruster_proxy::request_response::{HyperRequestWrapper, HyperResponseWrapper};
use cursive::{utils::{span::SpannedString, markup::StyledString}, theme::{Style, BaseColor, Effect}};
use bstr::ByteSlice;
use std::ffi::CString;

fn query_to_spanned(query_str: &str) -> SpannedString<Style> {
    let mut result = SpannedString::from(SpannedString::styled("?", BaseColor::Black.light()));

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
            result.append(SpannedString::styled("&", BaseColor::Black.light()));
        }
    }

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

        let hval = if let Ok(hval) = v.to_str() {
            StyledString::from(hval)
        }
        else {
            let mut spanned_hval = StyledString::from(v.as_bytes().to_str_lossy());
            spanned_hval.append("  ");
            spanned_hval.append(StyledString::styled("CANNOT FULLY ENCODE AS UTF-8", BaseColor::Black.light()));
            spanned_hval
        };

        headers_content.append(hval);
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