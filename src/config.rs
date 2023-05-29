use std::{fs, path};
use clap::{self, ArgMatches};
use serde_yaml as yml;
use shellexpand::tilde;
use serde::{Serialize, Deserialize};

use log::{LevelFilter, debug};
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config as L4R_Config, Root};

pub(crate) struct CrusterConfigError {
    error: String
}

impl Into<String> for CrusterConfigError {
    fn into(self) -> String {
        self.error
    }
}

impl<T> From<T> for CrusterConfigError
where  
    T: ToString 
{
    fn from(value: T) -> Self {
        Self { error: value.to_string() }
    }
}

pub(crate) enum CrusterMode {
    INTERACTIVE,
    DUMP(ArgMatches),
    CLI(ArgMatches)
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub(crate) struct Scope {
    pub(crate) include: Option<Vec<String>>,
    pub(crate) exclude: Option<Vec<String>>,
    pub(crate) strict: bool,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub(crate) struct Dump {
    pub(crate) enabled: bool,
    pub(crate) verbosity: u8,
    pub(crate) color: bool,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub(crate) struct Config {
    pub(crate) tls_key_name: String,
    pub(crate) tls_cer_name: String,
    pub(crate) address: String,
    pub(crate) port: u16,
    pub(crate) debug_file: Option<String>,
    pub(crate) project: Option<String>,
    pub(crate) scope: Option<Scope>,
    pub(crate) dump_mode: Option<Dump>
}

impl Default for Dump {
    fn default() -> Self {
        Dump {
            enabled: false,
            verbosity: 0,
            color: true
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            tls_cer_name: "cruster.cer".to_string(),
            tls_key_name: "cruster.key".to_string(),
            address: "127.0.0.1".to_string(),
            port: 8080_u16,
            debug_file: None,
            project: None,
            scope: None,
            dump_mode: None,
        }
    }
}

fn parse_cmd() -> clap::ArgMatches {
    let workplace_help = "Path to workplace, where data (configs, certs, projects, etc.) will be stored. Cannot be set by config file.";
    let config_help = "Path to config with YAML format. Cannot be set by config file.";
    let address_help = "Address for proxy to bind, default: 127.0.0.1";
    let port_help = "Port for proxy to listen to, default: 8080";
    let debug_file_help = "A file to write debug messages, mostly needed for development";
    let dump_help = "Enable non-interactive dumping mode: all communications will be shown in terminal output";
    let project_help = "Path to directory to store/load Cruster state. All files could be rewritten!";
    let strict_help = "If set, none of out-of-scope data will be written in storage, otherwise it will be just hidden from ui";
    let include_help = "Regex for URI to include in scope, i.e. ^https?://www\\.google\\.com/.*$. Option can repeat.";
    let exclude_help = "Regex for URI to exclude from scope, i.e. ^https?://www\\.google\\.com/.*$. Processed after include regex if any. Option can repeat.";
    let verbosity_help = "Verbosity in dump mode, ignored in intercative mode. 0: request/response first line, 
1: 0 + response headers, 2: 1 + request headers, 3: 2 + response body, 4: 3 + request body";
    let nc_help = "Disable colorizing in dump mode, ignored in interactive mode";
    let filter_help = "Filter pairs in specifyied bounds with regular expression in format of 're2'";
    let extract_help = "Extract pairs from range by attribute. parameter syntax: method=<name>|status=<value>|host=<prefix>|path=<prefix>";

    let matches = clap::Command::new("cruster")
        .version("0.7.0")
        .author("Andrey Ivanov<avangard.jazz@gmail.com>")
        .bin_name("cruster")
        .subcommand(
            clap::Command::new("interactive")
                .alias("i")
                .about("Default interactive Cruster mode. This mode will be used if none is specified")
        )
        .subcommand(
            clap::Command::new("dump")
                .alias("d")
                .about(dump_help)
                .arg(
                    clap::Arg::new("verbosity")
                        .short('v')
                        .action(clap::ArgAction::Count)
                        .help(verbosity_help)
                )
                .arg(
                    clap::Arg::new("no-color")
                        .long("nc")
                        .action(clap::ArgAction::SetTrue)
                        .help(nc_help)
                )
        )
        .subcommand(
            clap::Command::new("cli")
                .alias("c")
                .about("Cruster Command Line Interface")
                .subcommand_required(true)
                .arg_required_else_help(true)
                .subcommand(
                    clap::Command::new("http")
                        .about("Work with dumped HTTP data")
                        .alias("h")
                        .subcommand_required(true)
                        .subcommand(
                            clap::Command::new("show")
                                .about("Filter/Sort/Find HTTP data and print it")
                                .alias("s")
                                .arg_required_else_help(true)
                                .arg(
                                    clap::arg!(<INDEX> "range of line numbers or exact line number in file with stored HTTP data to print: n -- first n pairs, n-m -- pairs from n to m, a -- all stored pairs, n$ -- only Nth pair")
                                        .required(true)
                                )
                                .arg(
                                    clap::Arg::new("urls")
                                        .short('u')
                                        .long("urls")
                                        .action(clap::ArgAction::SetTrue)
                                        .help("Print only indexes and full URLs")
                                )
                                .arg(
                                    clap::Arg::new("pretty")
                                        .short('p')
                                        .long("pretty")
                                        .action(clap::ArgAction::SetTrue)
                                        .help("Print full formated requests and responses (if any)")
                                )
                                .arg(
                                    clap::Arg::new("raw")
                                        .short('r')
                                        .long("raw")
                                        .action(clap::ArgAction::SetTrue)
                                        .help("Print raw data as it was dumped in project")
                                )
                                .arg(
                                    clap::Arg::new("filter")
                                        .short('f')
                                        .long("filter")
                                        .help(filter_help)
                                )
                                .arg(
                                    clap::Arg::new("extract")
                                        .short('e')
                                        .long("extract")
                                        .value_name("ATTRIBUTE")
                                        .help(extract_help)
                                )
                        )
                )
                .subcommand(
                    clap::Command::new("repeater")
                        .alias("r")
                        .about("Launch repeater in CLI mode")
                        .subcommand_required(true)
                        .subcommand(
                            clap::Command::new("list")
                                .alias("l")
                                .about("List existing repeaters in current project")
                        )
                        .subcommand(
                            clap::Command::new("show")
                                .alias("s")
                                .about("Show specific repeater state verbously")
                                .arg_required_else_help(true)
                                .arg(
                                    clap::Arg::new("mark")
                                        .required(true)
                                        .help("Number or name of repeater to print")
                                )
                        )
                )
        )
        .arg(
            clap::Arg::new("workplace")
                .short('W')
                .long("workplace")
                // .default_value(default_workplace)
                .value_name("WORKPLACE_DIR")
                .help(workplace_help)
        )
        .arg(
            clap::Arg::new("config")
                .short('c')
                .long("config")
                // .default_value(default_config)
                .value_name("YAML_CONFIG")
                .help(config_help)
        )
        .arg(
            clap::Arg::new("address")
                .short('a')
                .long("address")
                .value_name("ADDR")
                .help(address_help)
        )
        .arg(
            clap::Arg::new("port")
                .short('p')
                .long("port")
                .value_name("PORT")
                .help(port_help)
        )
        .arg(
            clap::Arg::new("debug-file")
                .long("debug-file")
                .value_name("FILE-TO-WRITE")
                .help(debug_file_help)
        )
        .arg(
            clap::Arg::new("project")
                .long("project")
                .short('P')
                .value_name("PATH-TO-DIR")
                .help(project_help)
        )
        .arg(
            clap::Arg::new("strict-scope")
                .long("strict")
                .action(clap::ArgAction::SetTrue)
                .help(strict_help)
        )
        .arg(
            clap::Arg::new("include")
                .long("include-scope")
                .short('I')
                .value_name("REGEX")
                .action(clap::ArgAction::Append)
                .help(include_help)
        )
        .arg(
            clap::Arg::new("exclude")
                .long("exclude-scope")
                .short('E')
                .value_name("REGEX")
                .action(clap::ArgAction::Append)
                .help(exclude_help)
        )
        .get_matches();

    return matches;
}

// -----------------------------------------------------------------------------------------------//
#[allow(dead_code)]
pub(crate) fn handle_user_input() -> Result<(Config, CrusterMode), CrusterConfigError> {
    let matches = parse_cmd();
    let default_workplace = tilde("~/.cruster");
    let default_config = tilde("~/.cruster/config.yaml");

    let cmd = if let Some((subcmd, args)) = matches.subcommand() {
        match subcmd {
            "interactive" => CrusterMode::INTERACTIVE,
            "dump" => CrusterMode::DUMP(args.clone()),
            "cli" => CrusterMode::CLI(args.clone()),
            _ => unreachable!()
        }
    }
    else {
        CrusterMode::INTERACTIVE
    };

    let workplace_possible = matches.get_one::<String>("workplace");
    let config_possible = matches.get_one::<String>("config");

    let (workplace, config_name) = match (workplace_possible, config_possible) {
        (None, None) => {
            let workplace_path = path::Path::new(default_workplace.as_ref());
            if !workplace_path.exists() {
                fs::create_dir_all(workplace_path)?;
            }

            (default_workplace.to_string(), default_config.to_string())
        },
        (Some(workplace), None) => {
            let workplace_path = path::Path::new(workplace);
            if !workplace_path.exists() {
                return Err(
                    CrusterConfigError::from(format!("Could not find workplace dir at '{}'", workplace))
                );
            }

            let config_name = format!("{}/config.yaml", workplace);
            let config_path = path::Path::new(&config_name);
            if !config_path.exists() {
                return Err(
                    CrusterConfigError::from(format!("Could not find config file at unusual path '{}'", config_name))
                );
            }

            (workplace.to_string(), config_name)
        },
        (None, Some(config_name)) => {
            let config_path = path::Path::new(config_name);
            if !config_path.exists() {
                return Err(
                    CrusterConfigError::from(format!("Could not find config file at unusual path '{}'", config_name))
                );
            }

            let workplace_path = path::Path::new(default_workplace.as_ref());
            if !workplace_path.exists() {
                fs::create_dir_all(workplace_path)?;
            }

            (default_workplace.to_string(), config_name.to_string())
        },
        (Some(workplace), Some(config_name)) => {
            let workplace_path = path::Path::new(workplace);
            if !workplace_path.exists() {
                return Err(
                    CrusterConfigError::from(format!("Could not find workplace dir at '{}'", workplace))
                );
            }

            let config_path = path::Path::new(config_name);
            if !config_path.exists() {
                return Err(
                    CrusterConfigError::from(format!("Could not find config file at unusual path '{}'", config_name))
                );
            }

            (workplace.to_string(), config_name.to_string())
        }
    };

    let config_path = path::Path::new(config_name.as_str());
    let mut config = if config_path.exists() {
        let file = fs::File::open(config_name.as_str())?;
        let config_from_file: Config = yml::from_reader(file)?;
        config_from_file
    }
    else if config_name == default_config && workplace == default_workplace {
        let default_config = Config::default();
        let file = fs::File::create(config_name.as_str())?;
        let yaml_config = yml::to_value(&default_config)?;
        yml::to_writer(file, &yaml_config)?;
        default_config
    }
    else {
        unreachable!("You should not reach this place. If you somehow did it, write me to 'avangard.jazz@gmail.com'.");
    };

    if let Some(addr) = matches.get_one::<String>("address") {
        config.address = addr.to_string();
    }

    if let Some(port) = matches.get_one::<String>("port") {
        config.port = port.parse()?;
    }

    if let Some(dfile) = matches.get_one::<String>("debug-file") {
        let debug_file = resolve_path(&workplace, dfile, false)?;
        enable_debug(&debug_file);
        config.debug_file = Some(debug_file);
    }
    else if let Some(dfile) = &config.debug_file {
        let debug_file = resolve_path(&workplace, dfile, false)?;
        enable_debug(dfile);
        config.debug_file = Some(debug_file);
    }

    if let Some(store_path) = matches.get_one::<String>("project") {
        config.project = Some(resolve_path(&workplace, store_path, true)?);
        if ! path::Path::new(config.project.as_ref().unwrap()).is_dir() {
            fs::create_dir(config.project.as_ref().unwrap())?;
        }
    }
    else if let Some(store_path) = &config.project {
        config.project = Some(resolve_path(&workplace, store_path, true)?);
        if ! path::Path::new(config.project.as_ref().unwrap()).is_dir() {
            fs::create_dir(config.project.as_ref().unwrap())?;
        }
    }

    config.tls_cer_name = resolve_path(&workplace, &config.tls_cer_name, false)?;
    config.tls_key_name = resolve_path(&workplace, &config.tls_key_name, false)?;

    if matches.get_flag("strict-scope") {
        if let Some(scope) = config.scope.as_mut() {
            scope.strict = true;
        }
        else {
            config.scope = Some(Scope {
                include: None,
                exclude: None,
                strict: true
            });
        }
    }

    let include_scope = if let Some(include_re) = matches.get_many::<String>("include") {
        let res: Vec<String> = include_re
            .into_iter()
            .map(|v| {
                v.to_string()
            })
            .collect();
        
        Some(res)
    }
    else {
        None
    };

    let exclude_scope = if let Some(exclude_re) = matches.get_many::<String>("exclude") {
        let res: Vec<String> = exclude_re
            .into_iter()
            .map(|v| {
                v.to_string()
            })
            .collect();
        
        Some(res)
    }
    else {
        None
    };

    if include_scope.is_some() {
        if let None = &config.scope {
            config.scope = Some(Scope {
                include: include_scope,
                exclude: None,
                strict: false
            });
        }
        else {
            let scope_ref = config.scope.as_mut().unwrap();
            scope_ref.include = include_scope;
        }
    }

    if exclude_scope.is_some() {
        if let None = &config.scope {
            config.scope = Some(Scope { include: None, exclude: exclude_scope, strict: false });
        }
        else {
            let scope_ref = config.scope.as_mut().unwrap();
            scope_ref.exclude = exclude_scope;
        }
    }

    if let CrusterMode::DUMP(subcmd_args) = &cmd {
        if let Some(dm) = config.dump_mode.as_mut() {
            dm.enabled = true;
        }
        else {
            config.dump_mode = Some(
                Dump {
                    enabled: true,
                    ..Dump::default()
                }
            );
        }

        let mut verbosity = subcmd_args.get_count("verbosity");
        if verbosity > 0 {
            if let Some(dm) = config.dump_mode.as_mut() {
                if verbosity > 4 {
                    verbosity = 4;
                }

                dm.verbosity = verbosity;
            }
        }

        if subcmd_args.get_flag("no-color") {
            if let Some(dm) = config.dump_mode.as_mut() {
                dm.color = false;
            }
        }
    }

    Ok((config, cmd))
}

fn enable_debug(debug_file_path: &str) {
    let logfile = FileAppender::builder()
            .encoder(Box::new(PatternEncoder::new("[{d(%Y-%m-%d %H:%M:%S)}] {l} - {M} - {m}\n")))
            .build(debug_file_path)
            .unwrap();

        let log_config = L4R_Config::builder()
            .appender(Appender::builder().build("logfile", Box::new(logfile)))
            .build(Root::builder()
                    .appender("logfile")
                    .build(LevelFilter::Debug)).unwrap();

        log4rs::init_config(log_config).unwrap();
        debug!("Debugging enabled");
}

/// For existing files and dirs
// fn find_file_or_dir(base_path: &str, path: &str) -> Result<String, CrusterError> {
//     let fpath = path::Path::new(path);
//     if fpath.is_absolute() {
//         if fpath.exists() {
//             return Ok(path.to_string());
//         }
//         else {
//             return Err(
//                 CrusterError::ConfigError(
//                     format!("Could not find file or dir at absolute path '{}'", path)
//                 )
//             );
//         }
//     }
//     else {
//         if fpath.starts_with("./") && fpath.exists() {
//             return Ok(path.to_string());
//         }

//         let workspace_path = format!("{}/{}", base_path, path);
//         let wpath = path::Path::new(&workspace_path);
//         if wpath.exists() {
//             return Ok(workspace_path);
//         }
//         else {
//             if fpath.exists() {
//                 return Ok(path.to_string());
//             }
//             else {
//                 return Err(
//                     CrusterError::ConfigError(
//                         format!("Could not find file or dir at relative path '{}' neither in workplace nor working dir", path)
//                     )
//                 );
//             }
//         }
//     }
// }

/// Return such path state, which is accessbile with cruster
fn resolve_path(base_path: &str, path: &str, dir: bool) -> Result<String, CrusterConfigError> {
    let fpath = path::Path::new(path);
    if fpath.is_absolute() {
        return Ok(path.to_string());
    }
    else if fpath.starts_with("./") {
        if dir && ! std::path::Path::new(path).is_dir(){
            std::fs::create_dir(path)?;
        }
        else if !dir && ! std::path::Path::new(path).is_file() {
            std::fs::File::create(path)?;
        }
        
        let canonicalized = fs::canonicalize(fpath)?;
        let pth = canonicalized.to_str().unwrap().to_string();
        return Ok(pth);
    }
    else {
        return Ok(format!("{}/{}", base_path, path));
    }
}
