use crate::http_storage;
use crate::cli::CrusterCLIError;

use serde_json as json;
use std::io::BufRead;
use regex::Regex;

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
