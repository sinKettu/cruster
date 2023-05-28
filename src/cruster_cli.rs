#![allow(dead_code)]

mod config;

use newer_clap as clap;
use serde_yaml as yml;
use shellexpand::tilde;
use std::{fs, process::exit};

#[derive(Debug)]
struct CrusterCLIError {
    error: String
}

impl<T> From<T> for CrusterCLIError where T: ToString {
    fn from(e: T) -> Self {
        Self { error: e.to_string() }
    }
}

fn cli() -> clap::Command {
    clap::Command::new("cruster-cli")
        .about("Cruster Command Line Interface")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            clap::Command::new("http")
                .about("Work with dumped HTTP data")
                .subcommand_required(true)
                .subcommand(
                    clap::Command::new("show")
                        .about("Filter/Sort/Find HTTP data and print it")
                        .alias("s")
                        .arg_required_else_help(true)
                        .arg(
                            clap::arg!(<INDEX> "range or index in storage to print HTTP data: n -- first n pairs, n-m -- pairs from n to m, -m -- last m pairs, n! -- only Nth pair")
                                .required(true)
                        )
                        .arg(
                            clap::Arg::new("urls")
                                .short('u')
                                .long("urls")
                                .action(clap::ArgAction::SetTrue)
                                .help("Print only indexes and full URLs")
                        )
                )
        )
        .arg(
            clap::arg!(-c <CONFIG> "Path to cruster config")
                .default_value("~/.cruster/config.yaml")
        )
        .arg(
            clap::arg!(-p <PROJECT> "Path to project dir to work with (by default will try to get it from config)")
        )
}

fn main() -> Result<(), CrusterCLIError> {
    let command = cli().get_matches();

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
