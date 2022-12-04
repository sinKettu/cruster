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

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub(crate) struct Scope {
    pub(crate) include: Option<Vec<String>>,
    pub(crate) exclude: Option<Vec<String>>
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub(crate) struct Config {
    pub(crate) workplace: String,
    pub(crate) tls_key_name: String,
    pub(crate) tls_cer_name: String,
    pub(crate) config_name: String,
    pub(crate) address: String,
    pub(crate) port: u16,
    pub(crate) debug_file: Option<String>,
    pub(crate) dump_mode: bool,
    pub(crate) store: Option<String>,
    pub(crate) load: Option<String>,
    pub(crate) strict_scope: bool,
    pub(crate) scope: Option<Scope>,
}

impl Default for Config {
    fn default() -> Self {
        let expanded_path = tilde("~/.cruster/").to_string();
        Config {
            workplace: expanded_path.clone(),
            config_name: format!("{}{}", &expanded_path, "config.yaml"),
            tls_cer_name: format!("{}{}", &expanded_path, "cruster.cer"),
            tls_key_name: format!("{}{}", &expanded_path, "cruster.key"),
            address: "127.0.0.1".to_string(),
            port: 8080_u16,
            debug_file: None,
            dump_mode: false,
            store: None,
            load: None,
            strict_scope: false,
            scope: None,
        }
    }
}

// -----------------------------------------------------------------------------------------------//

pub(crate) fn handle_user_input() -> Result<Config, CrusterError> {
    let workplace_help = "Path to workplace, where data (configs, certs, projects, etc.) will be stored";
    let config_help = "Path to config with YAML format";
    let address_help = "Address for proxy to bind, default: 127.0.0.1";
    let port_help = "Port for proxy to listen to, default: 8080";
    let debug_file_help = "A file to write debug messages, mostly needed for development";
    let dump_help = "Enable non-interactive dumping mode: all communications will be shown in terminal output";
    let save_help = "Path to file to store proxy data. File will be rewritten!";
    let load_help = "Path to file to load previously stored data from";
    let strict_help = "If set, none of out-of-scope data will be written in storage, otherwise it will be just hidden from ui";
    let include_help = "Regex for URI to include in scope, i.e. ^https?://www\\.google\\.com/.*$. Option can repeat.";
    let exclude_help = "Regex for URI to exclude from scope, i.e. ^https?://www\\.google\\.com/.*$. Processed after include regex if any. Option can repeat.";

    let matches = App::new("Cruster")
        .version("0.4.4")
        .author("Andrey Ivanov<avangard.jazz@gmail.com>")
        .bin_name("cruster")
        .arg(
            Arg::with_name("workplace")
                .short("P")
                .long("workplace")
                .takes_value(true)
                .default_value("~/.cruster/")
                .value_name("WORKPLACE_DIR")
                .help(workplace_help)
        )
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .takes_value(true)
                .default_value("~/.cruster/config.yaml")
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
                .value_name("PATH-TO-FILE")
                .help(save_help)
        )
        .arg(
            Arg::with_name("load")
                .long("load")
                .short("-l")
                .takes_value(true)
                .value_name("PATH-TO-FILE")
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
        .get_matches();

    let workplace = tilde(
        matches
            .value_of("workplace")
            .ok_or(CrusterError::ConfigError("'--workplace' arg not found".to_owned()))?
    ).to_string();

    let config_name = tilde(matches
        .value_of("config")
        .ok_or(CrusterError::ConfigError("'--config' arg not found".to_owned()))?
    ).to_string();

    let workplace_path = path::Path::new(&workplace);
    if !workplace_path.exists() {
        fs::create_dir_all(workplace_path)?;
    }

    let config_path = path::Path::new(&config_name);
    let mut config = if config_path.exists() {
        let file = fs::File::open(&config_name)?;
        let config_from_file: Config = yml::from_reader(file)?;
        config_from_file
    }
    else {
        let default_config = Config {
            workplace,
            config_name,
            ..Default::default()
        };
        let file = fs::File::create(&default_config.config_name)?;
        let yaml_config = yml::to_value(&default_config)?;
        yml::to_writer(file, &yaml_config)?;
        default_config
    };

    if let Some(addr) = matches.value_of("address") {
        config.address = addr.to_string();
    }

    if let Some(port) = matches.value_of("port") {
        config.port = port.parse()?;
    }

    if let Some(dfile) = matches.value_of("debug-file") {
        enable_debug(dfile);
        config.debug_file = Some(dfile.to_string());
    }
    else if let Some(dfile) = &config.debug_file {
        enable_debug(dfile)
    }

    if let Some(store_path) = matches.value_of("store") {
        config.store = Some(store_path.to_string());
        fs::File::create(store_path).expect(&format!("Could not create file to store proxy data at '{}'", store_path));
    }
    else if let Some(store_path) = &config.store {
        fs::File::create(store_path).expect(&format!("Could not create file to store proxy data at '{}'", store_path));
    }

    if let Some(load_path) = matches.value_of("load") {
        config.load = Some(load_path.to_string());
    }

    if let Some(load_path) = &config.load {
        if ! path::Path::new(load_path).exists() {
            panic!("Could not find previously stored data at path '{}'", load_path);
        }
    }

    if matches.is_present("strict-scope") {
        config.strict_scope = true;
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
                exclude: None
            });
        }
        else {
            let scope_ref = config.scope.as_mut().unwrap();
            scope_ref.include = include_scope;
        }
    }

    if exclude_scope.is_some() {
        if let None = &config.scope {
            config.scope = Some(Scope { include: None, exclude: exclude_scope });
        }
        else {
            let scope_ref = config.scope.as_mut().unwrap();
            scope_ref.exclude = exclude_scope;
        }
    }

    if matches.is_present("dump-mode") || config.dump_mode {
        config.dump_mode = true;
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