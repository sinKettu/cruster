use clap::ArgMatches;

use super::list;
use crate::cli::CrusterCLIError;
use crate::siv_ui::repeater::RepeaterState;

#[derive(Debug)]
pub(crate) struct RepeaterShowSettings {
    pub(crate) name: Option<String>,
    pub(crate) number: Option<usize>,
    pub(crate) no_body: bool,
    pub(crate) no_cookie: bool,
}

impl TryFrom<&ArgMatches> for RepeaterShowSettings {
    type Error = CrusterCLIError;
    fn try_from(args: &ArgMatches) -> Result<Self, Self::Error> {
        let no_body = args.get_flag("no-body");
        let no_cookie = args.get_flag("no-cookie");
        let mark = args.get_one::<String>("mark").unwrap().to_string();
        if let Ok(number) = mark.parse::<usize>() {
            return Ok(RepeaterShowSettings {
                name: None,
                number: Some(number),
                no_body,
                no_cookie,
            });
        } else {
            return Ok(RepeaterShowSettings {
                name: Some(mark),
                number: None,
                no_body,
                no_cookie,
            });
        }
    }
}

pub(super) fn print_repeater_request_and_response(repeater: &RepeaterState, no_body: bool, no_cookie: bool) {
    let request = &repeater.request;
    let raw_rsp = repeater.response.get_content();
    let response = raw_rsp.source();

    println!("{}\n{}\n", super::format_output(request, no_body, no_cookie),
        super::format_output(response, no_body, no_cookie));
}

pub(crate) fn execute(settings: &RepeaterShowSettings, path: &str) -> Result<(), CrusterCLIError> {
    let (i, records) =
        super::operations::load_selected(path, settings.number, settings.name.as_deref())?;
    let repeater = RepeaterState::try_from(records[i].clone())?;
    list::print_repeater_state(&repeater, i);
    println!();
    print_repeater_request_and_response(&repeater, settings.no_body, settings.no_cookie);
    Ok(())
}
