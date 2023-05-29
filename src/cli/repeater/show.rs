use clap::ArgMatches;

use super::list;
use super::RepeaterIterator;
use crate::cli::CrusterCLIError;
use crate::siv_ui::repeater::RepeaterState;

#[derive(Debug)]
pub(crate) struct RepeaterShowSettings {
    pub(crate) name: Option<String>,
    pub(crate) number: Option<usize>
}

impl TryFrom<&ArgMatches> for RepeaterShowSettings {
    type Error = CrusterCLIError;
    fn try_from(args: &ArgMatches) -> Result<Self, Self::Error> {
        let mark = args.get_one::<String>("mark").unwrap().to_string();
        if let Ok(number) = mark.parse::<usize>() {
            return Ok(
                RepeaterShowSettings {
                    name: None,
                    number: Some(number)
                }
            );
        }
        else {
            return Ok(
                RepeaterShowSettings {
                    name: Some(mark),
                    number: None
                }
            );
        }
    }
}

pub(super) fn print_repeater_request_and_response(repeater: &RepeaterState) {
    let request = &repeater.request;
    let raw_rsp = repeater.response.get_content();
    let response = raw_rsp.source();

    println!("{}\n{}\n", request, response);
}

pub(crate) fn execute(settings: &RepeaterShowSettings, path: &str) -> Result<(), CrusterCLIError> {
    let repeater_iter = RepeaterIterator::new(path);
    for (i, repeater) in repeater_iter.enumerate() {
        if let Some(number) = settings.number.as_ref() {
            if &(i + 1) == number {
                list::print_repeater_state(&repeater, i);
                println!();
                print_repeater_request_and_response(&repeater);

                return Ok(());
            }

            continue;
        }

        if let Some(name) = settings.name.as_ref() {
            if &repeater.name == name {
                list::print_repeater_state(&repeater, i);
                println!();
                print_repeater_request_and_response(&repeater);

                return Ok(());
            }

            continue;
        }
    }

    if let Some(number) = settings.number {
        return Err(
            CrusterCLIError::from(format!("Could not find repeater with number '{}'", number))
        );
    }

    if let Some(name) = settings.name.as_ref() {
        return Err(
            CrusterCLIError::from(format!("Could not find repeater with name '{}'", name))
        );
    }

    unreachable!()
}