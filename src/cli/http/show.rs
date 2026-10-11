use crate::http_storage;
use crate::cli::CrusterCLIError;

use serde_json as json;
use std::io::BufRead;
use std::{borrow::Cow, ffi::CString};
use regex::Regex;
use bstr::ByteSlice;
use colored::{ColoredString, Colorize};

use regex;
use clap::ArgMatches;

pub(super) enum ExtractionKey {
    METHOD,
    HOST,
    PATH,
    STATUS,
}

pub(super) struct ExtractionAttribute {
    key: ExtractionKey,
    value: String,
}

impl TryFrom<&str> for ExtractionAttribute {
    type Error = CrusterCLIError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let splitted: Vec<&str> = value.split("=").collect();
        if splitted.len() != 2 {
            return Err(
                CrusterCLIError::from("Extraction attribute has unapropriate format. It must fit 'key=value'")
            )
        }

        let key = splitted[0].to_lowercase();
        let val = splitted[1];

        let extraction_key = match key.as_str() {
            "method" => {
                ExtractionKey::METHOD
            },
            "host" => {
                ExtractionKey::HOST
            },
            "path" => {
                ExtractionKey::PATH
            },
            "status" => {
                ExtractionKey::STATUS
            },
            _ => {
                return Err(
                    CrusterCLIError::from("Unexpected key in extraction attribute, key must be one of [method,host,path,status]")
                )
            }
        };

        return Ok(
            ExtractionAttribute {
                key: extraction_key,
                value: val.to_string()
            }
        );
    }
}

#[derive(Debug)]
pub(crate) struct HTTPTableRange {
    from: usize,
    to: usize,
    all: bool,
}

pub(crate) struct ShowSettings {
    pub(super) print_urls: bool,
    pub(super) pretty: bool,
    pub(super) raw: bool,
    pub(super) filter: Option<String>,
    pub(super) attribute: Option<ExtractionAttribute>,
    pub(super) index: Option<usize>
}

impl Default for ShowSettings {
    fn default() -> Self {
        ShowSettings {
            print_urls: false,
            pretty: false,
            raw: false,
            filter: None,
            attribute: None,
            index: None
        }
    }
}

pub(crate) fn parse_settings(args: &ArgMatches) -> Result<ShowSettings, CrusterCLIError> {
    let mut settings = ShowSettings::default();

    settings.print_urls = args.get_flag("urls");
    settings.pretty = args.get_flag("pretty");
    settings.raw = args.get_flag("raw");
    settings.filter = match args.get_one::<String>("filter") {
        Some(filter) => Some(filter.clone()),
        None => None
    };
    settings.attribute = match args.get_one::<String>("extract") {
        Some(attribute) => Some(ExtractionAttribute::try_from(attribute.as_str())?),
        None => None
    };
    settings.index = match args.get_one::<String>("index") {
        Some(index) => Some(index.to_string().parse()?),
        None => None
    };

    if settings.print_urls && settings.pretty {
        return Err(
            CrusterCLIError::from("Parameters '-u' and '-p' cannot be used at the same time")
        )
    }

    if settings.print_urls && settings.raw {
        return Err(
            CrusterCLIError::from("Parameters '-u' and '-r' cannot be used at the same time")
        )
    }

    if settings.raw && settings.pretty {
        return Err(
            CrusterCLIError::from("Parameters '-r' and '-p' cannot be used at the same time")
        )
    }

    if settings.attribute.is_some() && settings.raw {
        return Err(
            CrusterCLIError::from("Extraction by attribute ('-e') cannot be used with raw printing ('-r')")
        )
    }

    return Ok(settings);
}

pub(crate) fn parse_range(str_range: &str) -> Result<HTTPTableRange, CrusterCLIError> {
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

fn pretty_query(query: &str) -> String {
    query
        .trim_start_matches('?')
        .split('&')
        .map(|parameter| {
            let (key, value) = parameter.split_once('=').unwrap_or((parameter, ""));
            if parameter.contains('=') {
                format!("{}{}{}", key.bright_yellow(), "=".bright_black(), value.bright_green())
            }
            else {
                key.bright_yellow().to_string()
            }
        })
        .collect::<Vec<String>>()
        .join(&"&".bright_black().to_string())
}

fn pretty_headers(headers: &::http::HeaderMap) {
    let mut keys: Vec<&str> = headers.keys().map(|key| key.as_str()).collect();
    keys.sort();

    for key in keys {
        let values = headers
            .get_all(key)
            .iter()
            .map(|value| value.as_bytes().to_str_lossy())
            .collect::<Vec<Cow<str>>>()
            .join("; ");
        println!("{}{} {}", key.bright_blue(), ":".bright_black(), values);
    }
}

fn pretty_body(body: &[u8]) -> String {
    let body = body.to_str_lossy().to_string();
    if CString::new(body.as_bytes()).is_ok() {
        body
    }
    else {
        "--- INCOMPATIBLE SET OF BYTES ---".to_string()
    }
}

fn pretty_status(status: &str) -> ColoredString {
    match status.parse::<u16>().ok().map(|status| status / 100) {
        Some(2) => status.bright_green(),
        Some(3) => status.bright_cyan(),
        Some(4) => status.bright_yellow(),
        Some(5) => status.bright_red(),
        _ => status.bright_magenta(),
    }
}

fn print_pretty_request(request: &crate::cruster_proxy::request_response::HyperRequestWrapper) {
    let path = request
        .get_request_path_without_query()
        .unwrap_or_else(|_| request.get_request_path());
    let query = request.get_query();

    print!("{} {}", request.method.bold().bright_cyan(), path.bright_white());
    if let Some(query) = query {
        print!("{}{}", "?".bright_black(), pretty_query(&query));
    }
    println!(" {}", request.version.bright_black());
    pretty_headers(&request.headers);

    if !request.body.is_empty() {
        println!("\n{}", pretty_body(&request.body));
    }
}

fn print_pretty_response(response: &crate::cruster_proxy::request_response::HyperResponseWrapper) {
    println!("{} {}", response.version.bright_black(), pretty_status(&response.status));
    pretty_headers(&response.headers);

    if !response.body.is_empty() {
        println!("\n{}", pretty_body(&response.body));
    }
}

fn print_pretty(pair: &http_storage::RequestResponsePair) {
    println!("\n{}", format!(" HTTP exchange #{} ", pair.index).bold().on_bright_black());

    match (pair.request.as_ref(), pair.response.as_ref()) {
        (Some(request), Some(response)) => {
            println!("{}", "── REQUEST ──────────────────────────────────────────────".bright_blue());
            print_pretty_request(request);
            println!("\n{}", "── RESPONSE ─────────────────────────────────────────────".bright_magenta());
            print_pretty_response(response);
            println!("{}", "──────────────────────────────────────────────────────────".bright_black());
        },
        (Some(request), None) => {
            println!("{}", "── REQUEST ──────────────────────────────────────────────".bright_blue());
            print_pretty_request(request);
            println!("\n{}", "── RESPONSE ─────────────────────────────────────────────".bright_magenta());
            println!("{}", "<EMPTY RESPONSE>".bright_black());
            println!("{}", "──────────────────────────────────────────────────────────".bright_black());
        },
        (None, Some(response)) => {
            println!("{}", "── REQUEST ──────────────────────────────────────────────".bright_blue());
            println!("{}", "<EMPTY REQUEST>".bright_black());
            println!("\n{}", "── RESPONSE ─────────────────────────────────────────────".bright_magenta());
            print_pretty_response(response);
            println!("{}", "──────────────────────────────────────────────────────────".bright_black());
        }
        _ => unreachable!()
    }
}

fn print_pair(pair: &http_storage::RequestResponsePair, settings: &ShowSettings, header_if_any: bool) {
    if settings.print_urls {
        super::print_urls(pair);
    }
    else if settings.pretty {
        print_pretty(pair);
    }
    else {
        super::print_briefly(pair, header_if_any);
    }
}

fn matches_the_filter(pair: &http_storage::RequestResponsePair, re: &Regex) -> bool {
    let request_matched = if let Some(request) = pair.request.as_ref() {
        request.serach_with_re(re)
    }
    else {
        false
    };

    if request_matched {
        return true;
    }

    let response_matched = if let Some(response) = pair.response.as_ref() {
        response.serach_with_re(re)
    }
    else {
        false
    };

    return response_matched;
}

fn has_appropriate_attribute(pair: &http_storage::RequestResponsePair, attribute: &ExtractionAttribute) -> bool {
    match attribute.key {
        ExtractionKey::HOST => {
            if let Some(request) = pair.request.as_ref() {
                return request
                    .get_host()
                    .starts_with(&attribute.value);
            }
            else {
                return false;
            }
        },
        ExtractionKey::METHOD => {
            if let Some(request) = pair.request.as_ref() {
                return request.method
                    .to_lowercase()
                    .starts_with(&attribute.value.to_lowercase());
            }
            else {
                return false;
            }
        },
        ExtractionKey::PATH => {
            if let Some(request) = pair.request.as_ref() {
                return request
                    .get_request_path()
                    .starts_with(&attribute.value);
            }
            else {
                return false;
            }
        },
        ExtractionKey::STATUS => {
            if let Some(response) = pair.response.as_ref() {
                return response.status.starts_with(&attribute.value);
            }
            else {
                return false;
            }
        }
    }
}

pub(crate) fn execute(range: HTTPTableRange, http_storage: &str, settings: ShowSettings) -> Result<(), CrusterCLIError> {
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

    let filter_re = match settings.filter.as_ref() {
        Some(filter_str) => Some(Regex::new(filter_str)?),
        None => None
    };

    let mut first: bool = true;
    let fin = std::fs::File::open(http_storage)?;
    let fin_reader = std::io::BufReader::new(fin);

    let mut count = left_idx.saturating_sub(1);
    let mut found = false;
    for line in fin_reader.lines().skip(count) {
        let line_ptr = &line?;

        if settings.raw {
            if let Some(index) = settings.index {
                let re_str = format!(r#"^."index":{},"#, index);
                let re = Regex::new(&re_str)?;

                if let None = re.find(line_ptr) {
                    count += 1;
                    continue;
                }
            }

            if let Some(re) = filter_re.as_ref() {
                if re.find(line_ptr).is_some() {
                    found = true;
                    println!("{}", line_ptr);
                }
            }
            else {
                found = true;
                println!("{}", line_ptr);
            }
        }
        else {
            let serializable_data: http_storage::serializable::SerializableProxyData = json::from_str(line_ptr)?;
            let pair: http_storage::RequestResponsePair = serializable_data.try_into()?;

            if let Some(attr) = settings.attribute.as_ref() {
                if ! has_appropriate_attribute(&pair, attr) {
                    count += 1;
                    if count == right_idx {
                        break;
                    }

                    continue;
                }
            }

            if let Some(index) = settings.index {
                if pair.index != index {
                    count += 1;
                    continue;
                }
            }

            if let Some(re) = filter_re.as_ref() {
                if matches_the_filter(&pair, re) {
                    found = true;
                    print_pair(&pair, &settings, first);
                    if first {
                        first = false;
                    }        
                }
            }
            else {
                found = true;
                print_pair(&pair, &settings, first);
                if first {
                    first = false;
                }
            }
        }
        
        count += 1;
        if count == right_idx {
            break;
        }
    }

    if count == left_idx.saturating_sub(1) {
        return Err(CrusterCLIError::from("Left bound is out of range!"));
    }
    else if count < right_idx && !range.all {
        return Err(CrusterCLIError::from(format!("Could print only records from {} to {}", left_idx.saturating_sub(1), count)));
    }

    if !found {
        return Err(CrusterCLIError::from("Nothing is found"));
    }

    Ok(())
}
