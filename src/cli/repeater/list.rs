use std::io::{BufReader, BufRead};

use clap::ArgMatches;
use serde_json as json;

use crate::cli::CrusterCLIError;
use crate::siv_ui::repeater::{
    RepeaterState,
    RepeaterParameters,
    RepeaterStateSerializable,
};

pub(crate) struct RepeaterListSettings {
    pub(crate) verbose: bool
}

impl TryFrom<&ArgMatches> for RepeaterListSettings {
    type Error = CrusterCLIError;
    fn try_from(args: &ArgMatches) -> Result<Self, Self::Error> {
        let verbose = args.get_flag("verbose");
        return Ok(
            RepeaterListSettings {
                verbose
            }
        );
    }
}


pub(crate) fn execute(settings: &RepeaterListSettings, repeater_state_path: &str) -> Result<(), CrusterCLIError> {
    if ! std::path::Path::new(repeater_state_path).is_file() {
        return Err(
            CrusterCLIError::from(
                format!("Cannot find file with repeater's state at {}", repeater_state_path)
            )
        );
    }

    let fin = std::fs::File::open(repeater_state_path)?;
    let fin_reader = BufReader::new(fin);
    for (i, line) in fin_reader.lines().enumerate() {
        if let Ok(line) = line {
            let ser_repeater: RepeaterStateSerializable = json::from_str(&line)?;
            let repeater_state = RepeaterState::try_from(ser_repeater)?;

            println!("\n{:<18}: {}", "Number", i + 1);
            println!("{:<18}: {}", "Name", &repeater_state.name);
            println!("{:<18}: {}", "Host", &repeater_state.parameters.address);
            println!("{:<18}: {}", "HTTPS", &repeater_state.parameters.https);
            println!("{:<18}: {}", "Redirects", &repeater_state.parameters.redirects);
            if repeater_state.parameters.redirects {
                println!("{:<18}: {}", "Maximum redirects", repeater_state.parameters.max_redirects)
            }
        }
    }

    Ok(())
}