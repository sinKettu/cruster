use clap::ArgMatches;
use crate::{cli::CrusterCLIError, http_storage, siv_ui::repeater::{RepeaterState, RepeaterParameters}};
use std::io::{BufReader, BufRead};
use serde_json as json;
use cursive::views::TextContent;


pub(crate) struct RepeaterAddSettings {
    pub(crate) index: usize,
}

impl TryFrom<&ArgMatches> for RepeaterAddSettings {
    type Error = CrusterCLIError;
    fn try_from(args: &ArgMatches) -> Result<Self, Self::Error> {
        let mut settings = RepeaterAddSettings {
            index: usize::MAX
        };

        settings.index = args.get_one::<String>("index").unwrap().parse()?;

        return Ok(settings);
    }
}

pub(crate) fn exec(settings: &RepeaterAddSettings, http_path: &str, repeater_path: &str) -> Result<(), CrusterCLIError> {
    // Not very good way, but...
    let next_repeater_id = if std::path::Path::new(repeater_path).is_file() {
        super::RepeaterIterator::new(repeater_path).count()
    }
    else {
        0
    };


    let file = std::fs::File::open(http_path)?;
    let fin = BufReader::new(file);

    for raw_line in fin.lines() {
        let line = &raw_line?;

        let pair_ser: http_storage::serializable::SerializableProxyData = json::from_str(line)?;
        let pair: http_storage::RequestResponsePair = pair_ser.try_into()?;

        if pair.index != settings.index {
            continue;
        }

        let (request_str, headers, address, https) = if let Some(request) = pair.request.as_ref() {
            (request.to_string(), request.headers.clone(), request.get_hostname(), (request.get_scheme() == "https://"))
        }
        else {
            return Err(CrusterCLIError::from("Cannot create repeater from record with empty request"));
        };

        let response_str = if let Some(response) = pair.response.as_ref() {
            TextContent::new(response.to_string())
        }
        else {
            TextContent::new("")
        };

        let state = RepeaterState {
            name: format!("Repeater #{}", next_repeater_id),
            request: request_str,
            response: response_str,
            saved_headers: headers,
            redirects_reached: 0,
            parameters: RepeaterParameters {
                redirects: true,
                https,
                address,
                max_redirects: 10
            }
        };

        super::update_repeaters(repeater_path, &state, usize::MAX)?;
        break;
    }

    Ok(())
}