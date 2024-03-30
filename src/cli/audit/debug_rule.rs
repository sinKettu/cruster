use std::sync::Arc;

use clap::ArgMatches;

use crate::{audit::Rule, cli::CrusterCLIError, http_storage::RequestResponsePair};


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
    let rule = Rule::from_file(&conf.pth)?;
    let mut storage = crate::http_storage::HTTPStorage::default();
    storage.load(http_data_path)?;

    if let Some(index) = conf.pair_index {
        if let Some(pair) = storage.get_by_id(index) {
            let execution_state = rule.execute(pair).await;
            println!("{:#?}", execution_state);
        }
        else {
            return Err(CrusterCLIError::from(format!("no pair with index {} was found", index)));
        }
    }
    else {
        for pair in storage.into_iter() {
            let execution_state = rule.execute(pair).await;
            println!("{:#?}", execution_state);
        }
    }

    Ok(())
}
