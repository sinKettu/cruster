use crate::http_storage;
use crate::cli::CrusterCLIError;

use serde_json as json;
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

pub(crate) struct ExtractionAttribute {
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
    pub(crate) print_urls: bool,
    pub(crate) pretty: bool,
    pub(crate) no_body: bool,
    pub(crate) raw: bool,
    pub(crate) filter: Option<String>,
    pub(crate) attribute: Option<ExtractionAttribute>,
    pub(crate) index: Option<usize>
}

impl HTTPTableRange {
    pub(crate) fn selects_id(&self, index: Option<usize>) -> bool {
        index.is_some() || matches!(self.selection, HTTPTableSelection::Id(_))
    }

    /// Shared positional selection for history queries and audit snapshots.
    /// The ID lookup is lazy: unselected/malformed JSON is not decoded for ranges.
    pub(crate) fn select_positions(
        &self,
        len: usize,
        index: Option<usize>,
        mut id_at: impl FnMut(usize) -> Option<usize>,
    ) -> Vec<usize> {
        if let Some(id) = self.requested_id(index) {
            return (0..len).filter(|&position| id_at(position) == Some(id)).collect();
        }
        let (start, end) = match self.selection {
            HTTPTableSelection::First(count) => (0, count.min(len)),
            HTTPTableSelection::Range(from, to) => (from.saturating_sub(1).min(len), to.min(len)),
            HTTPTableSelection::Last(count) => (len.saturating_sub(count), len),
            HTTPTableSelection::All => (0, len),
            HTTPTableSelection::Id(_) => unreachable!("ID selections are handled before ranges"),
        };
        (start..end).collect()
    }

    fn requested_id(&self, index: Option<usize>) -> Option<usize> {
        index.or(match self.selection {
            HTTPTableSelection::Id(id) => Some(id),
            _ => None,
        })
    }
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
    let selects_single_id = range.selects_id(settings.index);

    // Selecting an exact pair is a detail-oriented operation, so it always
    // uses the full pretty representation regardless of presentation flags.
    if selects_single_id {
        settings.pretty = true;
        settings.raw = false;
        settings.print_urls = false;
    }

    let mut first = true;
    query(&range, http_storage, &settings, |raw, pair| {
        if let Some(pair) = pair {
            print_pair(pair, &settings, first);
            first = false;
        } else {
            println!("{}", raw);
        }
        Ok(())
    })
}

/// CLI history selection and filtering, independent of presentation and stdout.
/// The visitor keeps CLI output incremental even if a later selected row fails.
/// Raw rows deliberately bypass JSON/domain decoding, as in CLI show.
pub(crate) fn query(
    range: &HTTPTableRange,
    path: impl AsRef<std::path::Path>,
    settings: &ShowSettings,
    mut visit: impl FnMut(&str, Option<&http_storage::RequestResponsePair>) -> Result<(), CrusterCLIError>,
) -> Result<(), CrusterCLIError> {
    let filter_re = settings.filter.as_deref().map(Regex::new).transpose()?;
    let mut reader = std::io::BufReader::new(std::fs::File::open(path)?);
    let mut pending = Vec::new();
    let mut lines = Vec::new();
    while let Some(line) = super::read_history_line(&mut reader, &mut pending, false)? {
        lines.push(line.raw);
    }
    let requested_id = range.requested_id(settings.index);
    let selected_lines: Vec<_> = range.select_positions(lines.len(), settings.index, |position| {
        json::from_str::<http_storage::serializable::SerializableProxyData>(&lines[position])
            .ok().map(|data| data.index)
    }).into_iter().map(|position| &lines[position]).collect();

    if selected_lines.is_empty() {
        return if let Some(index) = requested_id {
            Err(CrusterCLIError::from(format!("HTTP pair with ID {} was not found", index)))
        }
        else {
            Err(CrusterCLIError::from("The selected range contains no HTTP pairs"))
        };
    }

    let mut found = false;
    let raw_mode = settings.raw && !range.selects_id(settings.index);
    for line in selected_lines {
        if raw_mode {
            if filter_re.as_ref().map_or(true, |re| re.is_match(line)) {
                visit(line, None)?;
                found = true;
            }
        } else {
            let pair = super::parse_history_pair(line)?;
            if settings.attribute.as_ref().map_or(false, |attr| !has_appropriate_attribute(&pair, attr)) {
                continue;
            }
            if filter_re.as_ref().map_or(false, |re| !matches_the_filter(&pair, re)) {
                continue;
            }
            visit(line, Some(&pair))?;
            found = true;
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
