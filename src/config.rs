use shellexpand::tilde;
use crate::utils::CrusterError;
use clap::{App, Arg};
use serde_yaml as yml;
use std::{fs, path};
use serde::{Serialize, Deserialize};
use simple_logging;
use log::{LevelFilter, debug};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub(crate) struct Config {
    pub(crate) workplace: String,
    pub(crate) tls_key_name: String,
    pub(crate) tls_cer_name: String,
    pub(crate) config_name: String,
    pub(crate) address: String,
    pub(crate) port: u16,
    pub(crate) debug_file: String,
    pub(crate) dump_mode: bool
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
            debug_file: "".to_string(),
            dump_mode: false
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

    let matches = App::new("Cruster")
        .version("0.2.4")
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
        config.debug_file = dfile.to_string();
        simple_logging::log_to_file(dfile, LevelFilter::Debug)
            .expect("Cannot configure debug logger to given file");
        debug!("Debugging enabled");
    }

    if matches.is_present("dump-mode") {
        config.dump_mode = true;
    }

    Ok(config)
}
