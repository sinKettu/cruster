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
    selection: HTTPTableSelection,
}

#[derive(Debug, PartialEq)]
enum HTTPTableSelection {
    First(usize),
    Range(usize, usize),
    Last(usize),
    Id(usize),
    All,
}

pub(crate) struct ShowSettings {
    pub(super) print_urls: bool,
    pub(super) pretty: bool,
    pub(super) no_body: bool,
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
            no_body: false,
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
    settings.no_body = args.get_flag("no-body");
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

    if settings.no_body && settings.raw {
        return Err(
            CrusterCLIError::from("Parameters '-b' and '-r' cannot be used at the same time")
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
    let first_re = regex::Regex::new(r"^\d+$")?;
    let range_re = regex::Regex::new(r"^(\d+)-(\d+)$")?;
    let negative_re = regex::Regex::new(r"^-(\d+)$")?;
    let prefixed_last_re = regex::Regex::new(r"^i(\d+)$")?;
    let suffixed_last_re = regex::Regex::new(r"^(\d+)i$")?;

    if str_range == "a" {
        return Ok(HTTPTableRange { selection: HTTPTableSelection::All });
    }

    if first_re.is_match(str_range) {
        return Ok(HTTPTableRange { selection: HTTPTableSelection::First(str_range.parse()?) });
    }

    if let Some(captures) = range_re.captures(str_range) {
        let from: usize = captures[1].parse()?;
        let to: usize = captures[2].parse()?;
        if from == 0 || to == 0 {
            return Err(CrusterCLIError::from("Range positions start at 1"));
        }
        if to < from {
            return Err(CrusterCLIError::from("Right bound of range cannot be lower than left one"));
        }
        return Ok(HTTPTableRange { selection: HTTPTableSelection::Range(from, to) });
    }

    if let Some(captures) = negative_re.captures(str_range) {
        return Ok(HTTPTableRange { selection: HTTPTableSelection::Last(captures[1].parse()?) });
    }

    for re in [&prefixed_last_re, &suffixed_last_re] {
        if let Some(captures) = re.captures(str_range) {
            return Ok(HTTPTableRange { selection: HTTPTableSelection::Id(captures[1].parse()?) });
        }
    }

    Err(CrusterCLIError::from("Range has wrong format; expected n, a, n-m, -n, iN, or Ni"))
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

fn print_pretty_request(
    request: &crate::cruster_proxy::request_response::HyperRequestWrapper,
    no_body: bool,
) {
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

    if !no_body && !request.body.is_empty() {
        println!("\n{}", pretty_body(&request.body));
    }
}

fn print_pretty_response(
    response: &crate::cruster_proxy::request_response::HyperResponseWrapper,
    no_body: bool,
) {
    println!("{} {}", response.version.bright_black(), pretty_status(&response.status));
    pretty_headers(&response.headers);

    if !no_body && !response.body.is_empty() {
        println!("\n{}", pretty_body(&response.body));
    }
}

fn print_pretty(pair: &http_storage::RequestResponsePair, no_body: bool) {
    println!("\n{}", format!(" HTTP exchange #{} ", pair.index).bold().on_bright_black());

    match (pair.request.as_ref(), pair.response.as_ref()) {
        (Some(request), Some(response)) => {
            println!("{}", "── REQUEST ──────────────────────────────────────────────".bright_blue());
            print_pretty_request(request, no_body);
            println!("\n{}", "── RESPONSE ─────────────────────────────────────────────".bright_magenta());
            print_pretty_response(response, no_body);
            println!("{}", "──────────────────────────────────────────────────────────".bright_black());
        },
        (Some(request), None) => {
            println!("{}", "── REQUEST ──────────────────────────────────────────────".bright_blue());
            print_pretty_request(request, no_body);
            println!("\n{}", "── RESPONSE ─────────────────────────────────────────────".bright_magenta());
            println!("{}", "<EMPTY RESPONSE>".bright_black());
            println!("{}", "──────────────────────────────────────────────────────────".bright_black());
        },
        (None, Some(response)) => {
            println!("{}", "── REQUEST ──────────────────────────────────────────────".bright_blue());
            println!("{}", "<EMPTY REQUEST>".bright_black());
            println!("\n{}", "── RESPONSE ─────────────────────────────────────────────".bright_magenta());
            print_pretty_response(response, no_body);
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
        print_pretty(pair, settings.no_body);
    }
    else {
        super::print_briefly(pair, header_if_any);
    }
}

pub(crate) fn matches_the_filter(pair: &http_storage::RequestResponsePair, re: &Regex) -> bool {
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

pub(crate) fn execute(range: HTTPTableRange, http_storage: &str, mut settings: ShowSettings) -> Result<(), CrusterCLIError> {
    let selects_single_id = settings.index.is_some()
        || matches!(range.selection, HTTPTableSelection::Id(_));

    // Selecting an exact pair is a detail-oriented operation, so it always
    // uses the full pretty representation regardless of presentation flags.
    if selects_single_id {
        settings.pretty = true;
        settings.raw = false;
        settings.print_urls = false;
    }

    let filter_re = match settings.filter.as_ref() {
        Some(filter_str) => Some(Regex::new(filter_str)?),
        None => None
    };

    let mut first: bool = true;
    let fin = std::fs::File::open(http_storage)?;
    let fin_reader = std::io::BufReader::new(fin);
    let lines: Vec<String> = fin_reader.lines().collect::<Result<_, _>>()?;

    let requested_id = settings.index.or_else(|| match range.selection {
        HTTPTableSelection::Id(index) => Some(index),
        _ => None,
    });

    let selected_lines: Vec<&String> = if let Some(index) = requested_id {
        // An explicit pair ID is searched in the complete storage and is not
        // restricted by the positional range.
        lines
            .iter()
            .filter(|line| {
                json::from_str::<http_storage::serializable::SerializableProxyData>(line)
                    .map(|data| data.index == index)
                    .unwrap_or(false)
            })
            .collect()
    }
    else {
        let (start, end) = match range.selection {
            HTTPTableSelection::First(count) => (0, count.min(lines.len())),
            HTTPTableSelection::Range(from, to) => {
                let start = from.saturating_sub(1).min(lines.len());
                let end = to.min(lines.len());
                (start, end)
            },
            HTTPTableSelection::Last(count) => (lines.len().saturating_sub(count), lines.len()),
            HTTPTableSelection::Id(_) => unreachable!("ID selections are handled before ranges"),
            HTTPTableSelection::All => (0, lines.len()),
        };

        if start >= end {
            Vec::new()
        }
        else {
            lines[start..end].iter().collect()
        }
    };

    if selected_lines.is_empty() {
        return if let Some(index) = requested_id {
            Err(CrusterCLIError::from(format!("HTTP pair with ID {} was not found", index)))
        }
        else {
            Err(CrusterCLIError::from("The selected range contains no HTTP pairs"))
        };
    }

    let mut found = false;
    for line_ptr in selected_lines {
        if settings.raw {
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
    }

    if !found {
        return Err(CrusterCLIError::from("Nothing is found"));
    }

    Ok(())
}

#[cfg(test)]
mod range_tests {
    use super::{parse_range, HTTPTableSelection};

    fn selection(value: &str) -> HTTPTableSelection {
        parse_range(value).unwrap().selection
    }

    #[test]
    fn parses_supported_range_forms() {
        assert_eq!(selection("100"), HTTPTableSelection::First(100));
        assert_eq!(selection("a"), HTTPTableSelection::All);
        assert_eq!(selection("10-20"), HTTPTableSelection::Range(10, 20));
        assert_eq!(selection("-20"), HTTPTableSelection::Last(20));
        assert_eq!(selection("i20"), HTTPTableSelection::Id(20));
        assert_eq!(selection("20i"), HTTPTableSelection::Id(20));
    }

    #[test]
    fn rejects_invalid_ranges() {
        assert!(parse_range("20-10").is_err());
        assert!(parse_range("0-10").is_err());
        assert!(parse_range("something").is_err());
    }
}
