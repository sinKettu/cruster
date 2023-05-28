use clap::{self, ArgMatches};
use std::fs;
use regex;

use crate::config;
use crate::http_storage;

#[derive(Debug)]
struct HTTPTableRange {
    from: usize,
    to: usize,
    only_one: bool
}

#[derive(Debug)]
pub struct CrusterCLIError {
    error: String
}

impl<T> From<T> for CrusterCLIError where T: ToString {
    fn from(e: T) -> Self {
        Self { error: e.to_string() }
    }
}

impl Into<String> for CrusterCLIError {
    fn into(self) -> String {
        self.error
    }
}

fn parse_range(str_range: &str) -> Result<HTTPTableRange, CrusterCLIError> {
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

fn show(range: HTTPTableRange, http_storage: &http_storage::HTTPStorage) -> Result<(), CrusterCLIError> {
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
            println!("{:?}", pair);
        }
    }

    Ok(())
}

pub(crate) fn launch(command: ArgMatches, config: config::Config) -> Result<(), CrusterCLIError> {
    let project = match config.project.as_ref() {
        Some(path) => {
            path.to_string()
        },
        None => {
            return Err(
                CrusterCLIError::from("Cruster CLI cannot work without project specified")
            )
        }
    };

    match command.subcommand() {
        Some(("http", subcommands)) => {
            let http_data_path = format!("{}/http.jsonl", &project);
            let http_data = fs::File::open(&http_data_path)?;

            match subcommands.subcommand() {
                Some(("show", args)) => {
                    let str_range = args.get_one::<String>("INDEX").unwrap();
                    let urls = args.get_flag("urls");
                    let range = parse_range(str_range)?;

                    let mut http_storage = http_storage::HTTPStorage::default();
                    http_storage.load(&http_data_path)?;

                    if let Err(err) = show(range, &http_storage) {
                        let err_msg: String = err.into();
                        eprintln!("ERROR: {}", err_msg);
                    }
                },
                _ => {}
            }
        },
        _ => {
            unreachable!()
        }
    }

    Ok(())
}
