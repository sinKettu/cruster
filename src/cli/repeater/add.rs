use crate::{cli::CrusterCLIError, http_storage};
use clap::ArgMatches;
use serde_json as json;
use std::io::{BufRead, BufReader};

pub(crate) struct RepeaterAddSettings {
    pub(crate) index: usize,
}

impl TryFrom<&ArgMatches> for RepeaterAddSettings {
    type Error = CrusterCLIError;
    fn try_from(args: &ArgMatches) -> Result<Self, Self::Error> {
        let mut settings = RepeaterAddSettings { index: usize::MAX };

        settings.index = args.get_one::<String>("index").unwrap().parse()?;

        return Ok(settings);
    }
}

pub(crate) fn exec(
    settings: &RepeaterAddSettings,
    http_path: &str,
    repeater_path: &str,
) -> Result<(), CrusterCLIError> {
    let records = super::read_repeaters(std::path::Path::new(repeater_path))?;
    let file = std::fs::File::open(http_path)?;
    for raw_line in BufReader::new(file).lines() {
        let pair_ser: http_storage::serializable::SerializableProxyData =
            json::from_str(&raw_line?)?;
        let pair: http_storage::RequestResponsePair = pair_ser.try_into()?;
        if pair.index != settings.index {
            continue;
        }
        let request = pair.request.as_ref().ok_or_else(|| {
            CrusterCLIError::from("Cannot create repeater from record with empty request")
        })?;
        let record = super::operations::from_history(
            request,
            pair.response.as_ref(),
            format!("Repeater #{}", records.len()),
            10,
        )?;
        super::update_repeaters(std::path::Path::new(repeater_path), &record, usize::MAX)?;
        break;
    }
    Ok(())
}
