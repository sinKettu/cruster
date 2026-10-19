use std::sync::Arc;

use clap::ArgMatches;
use log::{debug, LevelFilter};
use log4rs::{self, config::{Appender, Logger, Root}, encode::pattern::PatternEncoder};

use crate::{audit::{AuditRule, python::PythonRule}, cli::CrusterCLIError};


pub(crate) struct DebugRuleConfig {
    pth: String,
    pair_index: Option<usize>
}

impl TryFrom<&ArgMatches> for DebugRuleConfig {
    type Error = CrusterCLIError;
    fn try_from(value: &ArgMatches) -> Result<Self, Self::Error> {
        let pth = value.get_one::<String>("rule");
        let pair_index = match value.get_one::<usize>("http-pair-index") {
            Some(index) => {
                Some(index.to_owned())
            },
            None => {
                None
            }
        };

        if let Some(pth) = pth {
            return Ok(
                DebugRuleConfig { pth: pth.clone(), pair_index }
            )
        }
        else {
            return Err(CrusterCLIError::from("option 'rule' is required. See 'cruster clie debug-rule help'"));
        }
    }
}

pub(crate) async fn exec(conf: &DebugRuleConfig, http_data_path: &str) -> Result<(), CrusterCLIError> {
    if !conf.pth.ends_with(".py") {
        eprintln!(
            "Warning: YAML audit rule '{}' is temporarily disabled while its design is being reconsidered",
            conf.pth
        );
        return Err(CrusterCLIError::from(
            "YAML audit rules are temporarily unavailable in this version",
        ));
    }
    let rule = AuditRule::Python(PythonRule::from_file(&conf.pth)?);
    let mut storage = crate::http_storage::HTTPStorage::default();
    storage.load(http_data_path)?;

    let appender = log4rs::append::console::ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{m}\n")))
        .build();
    let log_config = log4rs::Config::builder()
        .appender(Appender::builder().build("console", Box::new(appender)))
        .logger(Logger::builder().build("cruster", LevelFilter::Debug))
        .build(Root::builder()
            .appender("console")
            .build(LevelFilter::Off)).unwrap();
    log4rs::init_config(log_config).unwrap();

    debug!("Rule id: {}", rule.get_id());

    if let Some(index) = conf.pair_index {
        if let Some(pair) = storage.get_by_id(index) {
            let execution_state = crate::audit::execution::execute_one(&rule, Arc::new(pair.clone())).await.state;
            println!("{:#?}", execution_state);
        }
        else {
            return Err(CrusterCLIError::from(format!("no pair with index {} was found", index)));
        }
    }
    else {
        for pair in storage.into_iter() {
            let execution_state = crate::audit::execution::execute_one(&rule, Arc::new(pair.clone())).await.state;
            println!("{:#?}", execution_state);
        }
    }

    Ok(())
}
