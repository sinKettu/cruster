use clap::ArgMatches;

use super::list;
use super::RepeaterIterator;
use crate::cli::CrusterCLIError;
use crate::siv_ui::repeater::RepeaterState;

#[derive(Debug)]
pub(crate) struct RepeaterShowSettings {
    pub(crate) name: Option<String>,
    pub(crate) number: Option<usize>,
    pub(crate) no_body: bool
}

impl TryFrom<&ArgMatches> for RepeaterShowSettings {
    type Error = CrusterCLIError;
    fn try_from(args: &ArgMatches) -> Result<Self, Self::Error> {
        let no_body = args.get_flag("no-body");
        let mark = args.get_one::<String>("mark").unwrap().to_string();
        if let Ok(number) = mark.parse::<usize>() {
            return Ok(
                RepeaterShowSettings {
                    name: None,
                    number: Some(number),
                    no_body
                }
            );
        }
        else {
            return Ok(
                RepeaterShowSettings {
                    name: Some(mark),
                    number: None,
                    no_body
                }
            );
        }
    }
}

pub(super) fn print_repeater_request_and_response(repeater: &RepeaterState, no_body: bool) {
    let request = &repeater.request;
    let raw_rsp = repeater.response.get_content();
    let response = raw_rsp.source();

    if no_body {
        let mut request_without_body = String::with_capacity(request.len());
        for line in request.split("\n") {
            if line.trim().is_empty() {
                request_without_body.push_str(line);
                request_without_body.push_str("\n");
                break
            }

            request_without_body.push_str(line);
            request_without_body.push_str("\n");
        }

        let mut response_without_body = String::with_capacity(response.len());
        for line in response.split("\n") {
            if line.trim().is_empty() {
                response_without_body.push_str(line);
                response_without_body.push_str("\n");
                break
            }

            response_without_body.push_str(line);
            response_without_body.push_str("\n");
        }

        println!("{}\n{}\n", request_without_body, response_without_body);
    } else {
        println!("{}\n{}\n", request, response);
    }
}

pub(crate) fn execute(settings: &RepeaterShowSettings, path: &str) -> Result<(), CrusterCLIError> {
    let repeater_iter = RepeaterIterator::new(path);
    for (i, repeater) in repeater_iter.enumerate() {
        if let Some(number) = settings.number.as_ref() {
            if &(i + 1) == number {
                list::print_repeater_state(&repeater, i);
                println!();
                print_repeater_request_and_response(&repeater, settings.no_body);

                return Ok(());
            }

            continue;
        }

        if let Some(name) = settings.name.as_ref() {
            if &repeater.name == name {
                list::print_repeater_state(&repeater, i);
                println!();
                print_repeater_request_and_response(&repeater, settings.no_body);

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