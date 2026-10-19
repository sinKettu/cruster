use clap::ArgMatches;

use super::list;
use crate::cli::CrusterCLIError;
use crate::siv_ui::repeater::RepeaterState;

#[derive(Debug)]
pub(crate) struct RepeaterShowSettings {
    pub(crate) name: Option<String>,
    pub(crate) number: Option<usize>,
    pub(crate) no_body: bool,
}

impl TryFrom<&ArgMatches> for RepeaterShowSettings {
    type Error = CrusterCLIError;
    fn try_from(args: &ArgMatches) -> Result<Self, Self::Error> {
        let no_body = args.get_flag("no-body");
        let mark = args.get_one::<String>("mark").unwrap().to_string();
        if let Ok(number) = mark.parse::<usize>() {
            return Ok(RepeaterShowSettings {
                name: None,
                number: Some(number),
                no_body,
            });
        } else {
            return Ok(RepeaterShowSettings {
                name: Some(mark),
                number: None,
                no_body,
            });
        }
    }
}

pub(super) fn print_repeater_request_and_response(repeater: &RepeaterState, no_body: bool) {
    let request = &repeater.request;
    let raw_rsp = repeater.response.get_content();
    let response = raw_rsp.source();

    if no_body {
        println!(
            "{}\n{}\n",
            super::trim_body(request),
            super::trim_body(response)
        );
    } else {
        println!("{}\n{}\n", request, response);
    }
}

pub(crate) fn execute(settings: &RepeaterShowSettings, path: &str) -> Result<(), CrusterCLIError> {
    let (i, records) =
        super::operations::load_selected(path, settings.number, settings.name.as_deref())?;
    let repeater = RepeaterState::try_from(records[i].clone())?;
    list::print_repeater_state(&repeater, i);
    println!();
    print_repeater_request_and_response(&repeater, settings.no_body);
    Ok(())
}
