use crate::http_storage;
use super::CrusterCLIError;

use std::cmp::min;

use regex;

#[derive(Debug)]
pub(super) struct HTTPTableRange {
    from: usize,
    to: usize,
    only_one: bool
}

pub(super) fn parse_range(str_range: &str) -> Result<HTTPTableRange, CrusterCLIError> {
    let right_bound_re = regex::Regex::new(r"^\d+$")?;
    let strict_index_re = regex::Regex::new(r"^\d+\$$")?;
    let left_bound_re = regex::Regex::new(r"^-\d+$")?;
    let range_re = regex::Regex::new(r"^\d+-\d+$")?;

    if right_bound_re.is_match(str_range) {
        return Ok(
            HTTPTableRange {
                from: 0,
                to: str_range.parse()?,
                only_one: false
            }
        );
    }

    if strict_index_re.is_match(str_range) {
        return Ok(
            HTTPTableRange {
                from: (&str_range[..(str_range.len() - 1)]).parse()?,
                to: 0,
                only_one: true
            }
        )
    }

    if left_bound_re.is_match(str_range) {
        let num = &str_range[1..];
        return Ok(
            HTTPTableRange {
                from: num.parse()?,
                to: usize::MAX,
                only_one: false
            }
        )
    }

    if range_re.is_match(str_range) {
        let parts: Vec<&str> = str_range.split("-").collect();
        return Ok(
            HTTPTableRange {
                from: parts[0].parse()?,
                to: parts[1].parse()?,
                only_one: false
            }
        );
    }

    return Err(
        CrusterCLIError::from("Range arg has wrong format")
    );
}

fn print_briefly(pair: &http_storage::RequestResponsePair) {
    let idx = pair.index;
    
    let (hostname, path) = if let Some(request) = pair.request.as_ref() {
        (request.get_hostname(), request.get_request_path())
    }
    else {
        ("<UNKNOWN>".to_string(), "<UNKNOWN>".to_string())
    };

    let (status, length) = if let Some(response) = pair.response.as_ref() {
        let status = response.status.split(" ").next().unwrap().to_string();
        let length = response.body.len().to_string();
        (status, length)
    }
    else {
        ("<UNKNOWN>".to_string(), "<UNKNOWN>".to_string())
    };


    //format!("{} {:>6} {}", "http".yellow(), hash, direction)
    println!("{:>6} {:>32} {:>70} {:>11} {:>15}", "ID", "HOSTNAME", "PATH", "STATUS", "LENGTH");
    println!(
        "{:>6} {:>32} {:>70} {:>11} {:>15}",
        idx,
        &hostname[..min(32, hostname.len())],
        &path[..min(70, path.len())],
        status,
        length
    );
}

pub(super) fn execute(range: HTTPTableRange, http_storage: &http_storage::HTTPStorage) -> Result<(), CrusterCLIError> {
    let (min_idx, max_idx) = http_storage.get_bounds();
    if range.only_one {
        let idx = range.from;
        if idx < min_idx || idx > max_idx {
            return Err(
                CrusterCLIError::from("Cannot show data: index is out of bounds")
            );
        }
        else {
            let pair = http_storage.get_by_id(idx);
            if let Some(pair) = pair {
                print_briefly(pair);
            }
            else {
                eprintln!("Pair with id {} does not exist", idx);
            }
        }
    }

    Ok(())
}
