use std::{fs, path};
use clap::{App, Arg};
use serde_yaml as yml;
use shellexpand::tilde;
use serde::{Serialize, Deserialize};

use log::{LevelFilter, debug};
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config as L4R_Config, Root};

use crate::utils::CrusterError;

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
    pub(crate) store: Option<String>,
    pub(crate) debug_file: Option<String>,
    pub(crate) load: Option<String>,
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
            store: None,
            load: None,
            scope: None,
            dump_mode: None,
        }
    }
}

// -----------------------------------------------------------------------------------------------//

pub(crate) fn handle_user_input() -> Result<Config, CrusterError> {
    let workplace_help = "Path to workplace, where data (configs, certs, projects, etc.) will be stored. Cannot be set by config file.";
    let config_help = "Path to config with YAML format. Cannot be set by config file.";
    let address_help = "Address for proxy to bind, default: 127.0.0.1";
    let port_help = "Port for proxy to listen to, default: 8080";
    let debug_file_help = "A file to write debug messages, mostly needed for development";
    let dump_help = "Enable non-interactive dumping mode: all communications will be shown in terminal output";
    let save_help = "Path to directory to store Cruster state. All files within will be rewritten!";
    let load_help = "Path to directory to load previously stored Cruster state";
    let strict_help = "If set, none of out-of-scope data will be written in storage, otherwise it will be just hidden from ui";
    let include_help = "Regex for URI to include in scope, i.e. ^https?://www\\.google\\.com/.*$. Option can repeat.";
    let exclude_help = "Regex for URI to exclude from scope, i.e. ^https?://www\\.google\\.com/.*$. Processed after include regex if any. Option can repeat.";
    let verbosity_help = "Verbosity in dump mode, ignored in intercative mode. 0: request/response first line, 
1: 0 + response headers, 2: 1 + request headers, 3: 2 + response body, 4: 3 + request body";
    
    let default_workplace = tilde("~/.cruster");
    let default_config = tilde("~/.cruster/config.yaml");

    let matches = App::new("Cruster")
        .version("0.6.0")
        .author("Andrey Ivanov<avangard.jazz@gmail.com>")
        .bin_name("cruster")
        .arg(
            Arg::with_name("workplace")
                .short("P")
                .long("workplace")
                .takes_value(true)
                // .default_value(default_workplace)
                .value_name("WORKPLACE_DIR")
                .help(workplace_help)
        )
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .takes_value(true)
                // .default_value(default_config)
                .value_name("YAML_CONFIG")
                .help(config_help)
        )
        .arg(
            Arg::with_name("address")
                .short("a")
                .long("address")
                .takes_value(true)
                .value_name("ADDR")
                .help(address_help)
        )
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .takes_value(true)
                .value_name("PORT")
                .help(port_help)
        )
        .arg(
            Arg::with_name("debug-file")
                .long("debug-file")
                .takes_value(true)
                .value_name("FILE-TO-WRITE")
                .help(debug_file_help)
        )
        .arg(
            Arg::with_name("dump-mode")
                .long("dump")
                .short("-d")
                .takes_value(false)
                .help(dump_help)
        )
        .arg(
            Arg::with_name("store")
                .long("store")
                .short("-s")
                .takes_value(true)
                .value_name("PATH-TO-DIR")
                .help(save_help)
        )
        .arg(
            Arg::with_name("load")
                .long("load")
                .short("-l")
                .takes_value(true)
                .value_name("PATH-TO-DIR")
                .help(load_help)
        )
        .arg(
            Arg::with_name("strict-scope")
                .long("strict")
                .help(strict_help)
        )
        .arg(
            Arg::with_name("include")
                .long("include-scope")
                .short("-I")
                .takes_value(true)
                .value_name("REGEX")
                .multiple(true)
                .number_of_values(1)
                .help(include_help)
        )
        .arg(
            Arg::with_name("exclude")
                .long("exclude-scope")
                .short("-E")
                .takes_value(true)
                .value_name("REGEX")
                .multiple(true)
                .number_of_values(1)
                .help(exclude_help)
        )
        .arg(
            Arg::with_name("verbosity")
                .short("v")
                .multiple(true)
                .help(verbosity_help)
        )
        .get_matches();

    let workplace_possible = matches.value_of("workplace");
    let config_possible = matches.value_of("config");

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
                    CrusterError::UndefinedError(
                        format!("Could not find workplace dir at '{}'", workplace)
                    )
                );
            }

            let config_name = format!("{}/config.yaml", workplace);
            let config_path = path::Path::new(&config_name);
            if !config_path.exists() {
                return Err(
                    CrusterError::UndefinedError(
                        format!("Could not find config file at unusual path '{}'", config_name)
                    )
                );
            }

            (workplace.to_string(), config_name)
        },
        (None, Some(config_name)) => {
            let config_path = path::Path::new(config_name);
            if !config_path.exists() {
                return Err(
                    CrusterError::UndefinedError(
                        format!("Could not find config file at unusual path '{}'", config_name)
                    )
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
                    CrusterError::UndefinedError(
                        format!("Could not find workplace dir at '{}'", workplace)
                    )
                );
            }

            let config_path = path::Path::new(config_name);
            if !config_path.exists() {
                return Err(
                    CrusterError::UndefinedError(
                        format!("Could not find config file at unusual path '{}'", config_name)
                    )
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

    if let Some(addr) = matches.value_of("address") {
        config.address = addr.to_string();
    }

    if let Some(port) = matches.value_of("port") {
        config.port = port.parse()?;
    }

    if let Some(dfile) = matches.value_of("debug-file") {
        let debug_file = resolve_path(&workplace, dfile)?;
        enable_debug(&debug_file);
        config.debug_file = Some(debug_file);
    }
    else if let Some(dfile) = &config.debug_file {
        let debug_file = resolve_path(&workplace, dfile)?;
        enable_debug(dfile);
        config.debug_file = Some(debug_file);
    }

    if let Some(store_path) = matches.value_of("store") {
        config.store = Some(resolve_path(&workplace, store_path)?);
        if ! path::Path::new(config.store.as_ref().unwrap()).is_dir() {
            fs::create_dir(config.store.as_ref().unwrap())?;
        }
    }
    else if let Some(store_path) = &config.store {
        config.store = Some(resolve_path(&workplace, store_path)?);
        if ! path::Path::new(config.store.as_ref().unwrap()).is_dir() {
            fs::create_dir(config.store.as_ref().unwrap())?;
        }
    }

    if let Some(load_path) = matches.value_of("load") {
        let lpath = find_file_or_dir(&workplace, load_path)?;
        if ! path::Path::new(&lpath).is_dir() {
            return Err(
                CrusterError::UndefinedError(
                    "Path to load data must be a dir, but it is not".to_string()
                )
            );
        }
        config.load = Some(lpath);
    }
    else if let Some(load_path) = &config.load {
        let lpath = find_file_or_dir(&workplace, load_path)?;
        if ! path::Path::new(&lpath).is_dir() {
            return Err(
                CrusterError::UndefinedError(
                    "Path to load data must be a dir, but it is not".to_string()
                )
            );
        }
        config.load = Some(lpath);
    }

    config.tls_cer_name = resolve_path(&workplace, &config.tls_cer_name)?;
    config.tls_key_name = resolve_path(&workplace, &config.tls_key_name)?;

    if matches.is_present("strict-scope") {
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

    let include_scope = if let Some(include_re) = matches.values_of("include") {
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

    let exclude_scope = if let Some(exclude_re) = matches.values_of("exclude") {
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

    if matches.is_present("dump-mode") {
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
    }

    if matches.is_present("verbosity") {
        if let Some(dm) = config.dump_mode.as_mut() {
            let mut count = matches.occurrences_of("verbosity");
            if count > 4 {
                count = 4;
            }

            dm.verbosity = count as u8;
        }
    }

    Ok(config)
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
fn find_file_or_dir(base_path: &str, path: &str) -> Result<String, CrusterError> {
    let fpath = path::Path::new(path);
    if fpath.is_absolute() {
        if fpath.exists() {
            return Ok(path.to_string());
        }
        else {
            return Err(
                CrusterError::ConfigError(
                    format!("Could not find file or dir at absolute path '{}'", path)
                )
            );
        }
    }
    else {
        if fpath.starts_with("./") && fpath.exists() {
            return Ok(path.to_string());
        }

        let workspace_path = format!("{}/{}", base_path, path);
        let wpath = path::Path::new(&workspace_path);
        if wpath.exists() {
            return Ok(workspace_path);
        }
        else {
            if fpath.exists() {
                return Ok(path.to_string());
            }
            else {
                return Err(
                    CrusterError::ConfigError(
                        format!("Could not find file or dir at relative path '{}' neither in workplace nor working dir", path)
                    )
                );
            }
        }
    }
}

/// Return such path state, which is accessbile with cruster
fn resolve_path(base_path: &str, path: &str) -> Result<String, CrusterError> {
    let fpath = path::Path::new(path);
    if fpath.is_absolute() {
        return Ok(path.to_string());
    }
    else if fpath.starts_with("./") {
        let canonicalized = fs::canonicalize(fpath)?;
        let pth = canonicalized.to_str().unwrap().to_string();
        return Ok(pth);
    }
    else {
        return Ok(format!("{}/{}", base_path, path));
    }
}