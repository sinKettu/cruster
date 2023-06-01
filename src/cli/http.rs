use crate::http_storage;
use std::cmp::min;

pub(super) mod show;
pub(super) mod follow;

pub(crate) fn print_briefly(pair: &http_storage::RequestResponsePair, with_header: bool) {
    let idx = pair.index;
    
    let (hostname, path, method) = if let Some(request) = pair.request.as_ref() {
        (request.get_hostname(), request.get_request_path(), request.method.clone())
    }
    else {
        ("<UNKNOWN>".to_string(), "<UNKNOWN>".to_string(), "<UNKNOWN>".to_string())
    };

    let (status, length) = if let Some(response) = pair.response.as_ref() {
        let status = response.status.split(" ").next().unwrap().to_string();
        let length = response.body.len().to_string();
        (status, length)
    }
    else {
        ("<UNKNOWN>".to_string(), "<UNKNOWN>".to_string())
    };

    if with_header {
        println!("{:>6} {:>8} {:>32} {:>70} {:>11} {:>15}\n", "ID", "METHOD", "HOSTNAME", "PATH", "STATUS", "LENGTH");
    }

    println!(
        "{:>6} {:>8} {:>32} {:<70} {:>11} {:>15}",
        idx,
        &method[..min(8, method.len())],
        &hostname[..min(32, hostname.len())],
        &path[..min(70, path.len())],
        status,
        length
    );
}

pub(crate) fn print_urls(pair: &http_storage::RequestResponsePair) {
    if let Some(request) = pair.request.as_ref() {
        println!(
            "{:>6} {}",
            pair.index,
            request.uri
        )
    }
    else {
        println!(
            "{:>6} {}",
            pair.index,
            "<NONE>"
        )
    }
}
