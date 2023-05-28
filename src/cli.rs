use clap::{self, ArgMatches};
use serde_yaml as yml;
use shellexpand::tilde;
use std::{fs, process::exit};
use crate::config;

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

pub fn launch(command: ArgMatches) -> Result<(), CrusterCLIError> {
    let config_path = tilde(command.get_one::<String>("CONFIG").unwrap());
    let possible_proj_path = command.get_one::<String>("PROJECT");
    let fin = fs::File::open(config_path.as_ref())?;
    let config: config::Config = yml::from_reader(fin)?;

    let project = if let Some(proj_path) = possible_proj_path {
        proj_path.to_string()
    }
    else {
        if let Some(proj_path) = config.project.as_ref() {
            proj_path.to_string()
        }
        else {
            eprintln!("Cannot find project to work with. Specify it neither with '-p' flag or in config");
            exit(1);
        }
    };

    match command.subcommand() {
        Some(("http", subcommands)) => {
            let http_data_path = format!("{}/http.jsonl", &project);
            let http_data = fs::File::open(&http_data_path)?;

            match subcommands.subcommand() {
                Some(("show", args)) => {
                    // (?P<left_bound>\d+)(?P<range_or_strict>[-!])?(?P<right_bound>\d+)?
                    let range = args.get_one::<String>("INDEX").unwrap();
                    let urls = args.get_flag("urls");

                    println!("INDEX:{:?} URLS:{:?}", range, urls);
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
