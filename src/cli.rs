mod http;
mod repeater;
mod audit;

use clap::{self, ArgMatches};

use crate::config;
use std::process::exit;

use self::audit::debug_rule::DebugRuleConfig;

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

pub(crate) async fn launch(command: ArgMatches, config: config::Config, audit_conf: config::AuditConfig) -> Result<(), CrusterCLIError> {
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
                    let range = http::show::parse_range(str_range)?;
                    let settings = http::show::parse_settings(args)?;

                    if let Err(err) = http::show::execute(range, &http_data_path, settings) {
                        let err_msg: String = err.into();
                        eprintln!("Error occured while http::show executed: {}", err_msg);
                        exit(1);
                    }
                },
                Some(("follow", args)) => {
                    let settings = http::follow::HttpFollowSettings::try_from(args)?;
                    if let Err(err) = http::follow::exec(&settings, &http_data_path) {
                        let err_msg: String = err.into();
                        eprintln!("Error occured while http::follow executed: {}", err_msg);
                        exit(8);
                    }
                }
                _ => {}
            }
        },
        Some(("repeater", subcommands)) => {
            let repeater_state_path = format!("{}/repeater.jsonl", &project);

            match subcommands.subcommand() {
                Some(("list", _)) => {
                    if let Err(err) = repeater::list::execute(&repeater_state_path) {
                        let err_str: String = err.into();
                        eprintln!("Error occured while repeater::list executed: {}", err_str);
                        exit(2);
                    }
                },
                Some(("show", args)) => {
                    let settings = repeater::show::RepeaterShowSettings::try_from(args)?;
                    if let Err(err) = repeater::show::execute(&settings, &repeater_state_path) {
                        let err_str: String = err.into();
                        eprintln!("Error occured while repeater::show executed: {}", err_str);
                        exit(3);
                    }
                },
                Some(("exec", args)) => {
                    if config.editor.is_none() {
                        eprintln!("Error: to use CLI repeater use must specify external text editor with 'cruster -e' option or in config");
                        exit(4);
                    }

                    let settings = repeater::exec::RepeaterExecSettings::try_from(args)?;
                    let editor = config.editor.as_ref().unwrap();
                    if let Err(err) = repeater::exec::execute(&settings, &repeater_state_path, editor).await {
                        let err_str: String = err.into();
                        eprintln!("Error occured while repeater::exec executed: {}", err_str);
                        exit(5);
                    }
                },
                Some(("edit", args)) => {
                    let settings = repeater::edit::RepeaterEditSettings::try_from(args)?;
                    if let Err(err) = repeater::edit::execute(&settings, &repeater_state_path) {
                        let err_str: String = err.into();
                        eprintln!("Error occured while repeater::edit executed: {}", err_str);
                        exit(6);
                    }
                },
                Some(("add", args)) => {
                    let http_data_path = format!("{}/http.jsonl", &project);
                    let settings = repeater::add::RepeaterAddSettings::try_from(args)?;
                    if let Err(err) = repeater::add::exec(&settings, &http_data_path, &repeater_state_path) {
                        let err_str: String = err.into();
                        eprintln!("Error occured while repeater::add executed: {}", err_str);
                        exit(7);
                    }
                }
                _ => unreachable!()
            }
        },
        Some(("audit", subcommands)) => {
            let http_data_path = format!("{}/http.jsonl", &project);
            match subcommands.subcommand() {
                Some(("run", _args)) => {
                    if let Err(err) = audit::run::exec(&audit_conf) {
                        let err_str: String = err.into();
                        eprintln!("Error occured  while audit::run executed: {}", err_str);
                        exit(8);
                    }
                },
                Some(("test", _args)) => {
                    let arg = _args.get_one::<String>("arg").unwrap();
                    if let Err(err) = audit::test::exec(arg, &http_data_path, &audit_conf).await {
                        let err_str: String = err.into();
                        eprintln!("Error occured while audit::test executed: {}", err_str);
                        exit(8);
                    }
                },
                Some(("debug-rule", _args)) => {
                    let conf = DebugRuleConfig::try_from(_args)?;
                    if let Err(err) = audit::debug_rule::exec(&conf, &http_data_path).await {
                        let err_str: String = err.into();
                        eprintln!("Error occured while audit::debug_rule executed: {}", err_str);
                        exit(9);
                    }
                },
                _ => {
                    unreachable!()
                }
            }
        }
        _ => {
            unreachable!()
        }
    }

    Ok(())
}
