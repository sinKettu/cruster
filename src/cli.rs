mod show;

use clap::{self, ArgMatches};

use crate::config;
use crate::http_storage;

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

            match subcommands.subcommand() {
                Some(("show", args)) => {
                    let str_range = args.get_one::<String>("INDEX").unwrap();
                    // let urls = args.get_flag("urls");
                    let range = show::parse_range(str_range)?;
                    let settings = show::parse_settings(args)?;

                    let mut http_storage = http_storage::HTTPStorage::default();
                    http_storage.load(&http_data_path)?;

                    if let Err(err) = show::execute(range, &http_storage, settings) {
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
