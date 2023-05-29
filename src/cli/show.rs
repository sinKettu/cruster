use crate::http_storage;
use super::CrusterCLIError;

use serde_json as json;
use std::{cmp::min, io::BufRead};

use regex;
use clap::ArgMatches;

#[derive(Debug)]
pub(super) struct HTTPTableRange {
    from: usize,
    to: usize,
    all: bool,
}

pub(super) struct ShowSettings {
    pub(super) print_urls: bool,
    pub(super) pretty: bool,
    pub(super) raw: bool,
}

impl Default for ShowSettings {
    fn default() -> Self {
        ShowSettings {
            print_urls: false,
            pretty: false,
            raw: false,
        }
    }
}

pub(super) fn parse_settings(args: &ArgMatches) -> Result<ShowSettings, super::CrusterCLIError> {
    let mut settings = ShowSettings::default();

    settings.print_urls = args.get_flag("urls");
    settings.pretty = args.get_flag("pretty");
    settings.raw = args.get_flag("raw");

    if settings.print_urls && settings.pretty {
        return Err(
            super::CrusterCLIError::from("Parameters '-u' and '-p' cannot be used at the same time")
        )
    }

    if settings.print_urls && settings.raw {
        return Err(
            super::CrusterCLIError::from("Parameters '-u' and '-r' cannot be used at the same time")
        )
    }

    if settings.raw && settings.pretty {
        return Err(
            super::CrusterCLIError::from("Parameters '-r' and '-p' cannot be used at the same time")
        )
    }

    return Ok(settings);
}

pub(super) fn parse_range(str_range: &str) -> Result<HTTPTableRange, CrusterCLIError> {
    let right_bound_re = regex::Regex::new(r"^\d+$")?;
    let strict_index_re = regex::Regex::new(r"^\d+\$$")?;
    let range_re = regex::Regex::new(r"^\d+-\d+$")?;

    if right_bound_re.is_match(str_range) {
        return Ok(
            HTTPTableRange {
                from: 0,
                to: str_range.parse()?,
                all: false
            }
        );
    }

    if strict_index_re.is_match(str_range) {
        return Ok(
            HTTPTableRange {
                from: (&str_range[..(str_range.len() - 1)]).parse()?,
                to: (&str_range[..(str_range.len() - 1)]).parse()?,
                all: false
            }
        )
    }

    if range_re.is_match(str_range) {
        let parts: Vec<&str> = str_range.split("-").collect();
        return Ok(
            HTTPTableRange {
                from: parts[0].parse()?,
                to: parts[1].parse()?,
                all: false
            }
        );
    }

    if str_range == "a" {
        return Ok(
            HTTPTableRange {
                from: 0,
                to: 0,
                all: true
            }
        )
    }

    return Err(
        CrusterCLIError::from("Range arg has wrong format")
    );
}

fn print_briefly(pair: &http_storage::RequestResponsePair, with_header: bool) {
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

    if with_header {
        println!("{:>6} {:>32} {:>70} {:>11} {:>15}\n", "ID", "HOSTNAME", "PATH", "STATUS", "LENGTH");
    }

    println!(
        "{:>6} {:>32} {:<70} {:>11} {:>15}",
        idx,
        &hostname[..min(32, hostname.len())],
        &path[..min(70, path.len())],
        status,
        length
    );
}

fn print_urls(pair: &http_storage::RequestResponsePair) {
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

fn print_pretty(pair: &http_storage::RequestResponsePair) {
    println!("----------------------");
    println!("   {}", pair.index);
    println!("----------------------\n");

    match (pair.request.as_ref(), pair.response.as_ref()) {
        (Some(request), Some(response)) => {
            println!("{}", request.to_string());
            println!();
            println!("{}", response.to_string());
            println!();
        },
        (Some(request), None) => {
            println!("{}", request.to_string());
            println!();
            println!("\n<EMPTY RESPONSE>\n");
        },
        (None, Some(response)) => {
            println!("<EMPTY REQUEST>\n");
            println!("{}", response.to_string());
            println!();
        }
        _ => unreachable!()
    }
}

fn print_pair(pair: &http_storage::RequestResponsePair, settings: &ShowSettings, header_if_any: bool) {
    if settings.print_urls {
        print_urls(pair);
    }
    else if settings.pretty {
        print_pretty(pair);
    }
    else {
        print_briefly(pair, header_if_any);
    }
}

pub(super) fn execute(range: HTTPTableRange, http_storage: &str, settings: ShowSettings) -> Result<(), CrusterCLIError> {
    if range.to < range.from {
        return Err(
            CrusterCLIError::from("Right bound of range cannot be lower than left one")
        );
    }

    let (left_idx, right_idx) = if range.all {
        (0_usize, usize::MAX)
    }
    else {
        (range.from, range.to)
    };

    let mut first: bool = true;
    let fin = std::fs::File::open(http_storage)?;
    let fin_reader = std::io::BufReader::new(fin);

    let mut count = left_idx.saturating_sub(1);
    for line in fin_reader.lines().skip(count) {
        let line_ptr = &line?;
        if settings.raw {
            println!("{}", line_ptr);
        }
        else {
            let serializable_data: http_storage::serializable::SerializableProxyData = json::from_str(line_ptr)?;
            let pair: http_storage::RequestResponsePair = serializable_data.try_into()?;

            print_pair(&pair, &settings, first);
            if first {
                first = false;
            }
        }
        
        count += 1;
        if count == right_idx {
            break;
        }
    }

    if count < right_idx && !range.all {
        eprintln!("\nCould print only {} records from {}", count - left_idx, right_idx - left_idx);
    }

    Ok(())
}
