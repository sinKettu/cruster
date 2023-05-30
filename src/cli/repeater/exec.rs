use std::{process, io::Read};
use std::io::Write;
use rand::{distributions::Alphanumeric, Rng};

use clap::ArgMatches;
use reqwest::{self, Request, Response};

use super::RepeaterIterator;
use crate::cli::CrusterCLIError;
use crate::siv_ui::repeater::{RepeaterState, RepeaterParameters};
use cursive::views::TextContent;


pub(crate) struct RepeaterExecSettings {
    pub(crate) name: Option<String>,
    pub(crate) number: Option<usize>,
    pub(crate) force: bool
}

impl TryFrom<&ArgMatches> for RepeaterExecSettings {
    type Error = CrusterCLIError;
    fn try_from(args: &ArgMatches) -> Result<Self, Self::Error> {
        let mut settings = RepeaterExecSettings {
            name: None,
            number: None,
            force: false
        };

        let mark = args.get_one::<String>("mark").unwrap().to_string();
        if let Ok(number) = mark.parse::<usize>() {
            settings.number = Some(number);
        }
        else {
            settings.name = Some(mark);
        }

        settings.force = args.get_flag("force");

        if settings.name.is_none() && settings.number.is_none() {
            return Err(
                CrusterCLIError::from("Use must specify number or name of repeater to work with")
            )
        }

        return Ok(settings);
    }
}

fn send_request(request: Request, params: &RepeaterParameters) -> Result<Response, CrusterCLIError> {
    

    todo!()
}

fn open_editor(editor: &str, request: String) -> Result<String, CrusterCLIError> {
    let tmp_path = format!(
        "/tmp/cruster-repeater-{}.txt",
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect::<String>()
    );

    let mut fout = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(&tmp_path)?;

    let _ = fout.write(request.as_bytes())?;
    drop(fout);

    let _status = process::Command::new(editor).arg(&tmp_path).status()?;

    let mut fin = std::fs::File::open(&tmp_path)?;
    let mut edited_request = String::with_capacity(request.len() + 100);
    fin.read_to_string(&mut edited_request)?;

    std::fs::remove_file(tmp_path)?;

    return  Ok(edited_request);
}

fn get_ready_request(repeater: &mut RepeaterState, editor: &str, force: bool) -> Result<Request, CrusterCLIError> {
    if force {
        return Ok(repeater.make_reqwest()?);
    }
    else {
        repeater.request = open_editor(editor, repeater.request.clone())?;
        return Ok(repeater.make_reqwest()?);
    };
}

fn handle_repeater(mut repeater: &mut RepeaterState, number: usize, path: &str, editor: &str, settings: &RepeaterExecSettings) -> Result<(), CrusterCLIError> {
    let request = get_ready_request(&mut repeater, editor, settings.force)?;
    super::update_repeaters(path, &repeater, number.to_owned())?;

    let response = send_request(request, &repeater.parameters)?;
    let wrapper = tokio::runtime::Runtime::new()?.block_on(
        crate::cruster_proxy::request_response::HyperResponseWrapper::from_reqwest(response)
    )?;

    let response_str = wrapper.to_string();
    repeater.response = TextContent::new(response_str.clone());
    super::update_repeaters(path, &repeater, number.to_owned())?;

    return Ok(())
}

pub(crate) fn execute(settings: &RepeaterExecSettings, path: &str, editor: &str) -> Result<(), CrusterCLIError> {
    let repeater_iter = RepeaterIterator::new(path);
    for (i, mut repeater) in repeater_iter.enumerate() {
        if let Some(number) = settings.number.as_ref() {
            if &(i + 1) == number {
                return handle_repeater(&mut repeater, i, path, editor, settings);
            }

            continue;
        }

        if let Some(name) = settings.name.as_ref() {
            if &repeater.name == name {
                return handle_repeater(&mut repeater, i, path, editor, settings);
            }

            continue;
        }
    }

    Err(
        CrusterCLIError::from("Cannot find repeater by specified mark (number/name)")
    )
}