use rand::{distributions::Alphanumeric, Rng};
use std::io::Write;
use std::{io::Read, process};

use clap::ArgMatches;
use cursive::views::TextContent;
use reqwest::Request;

use crate::cli::CrusterCLIError;
use crate::siv_ui::repeater::RepeaterState;

pub(crate) struct RepeaterExecSettings {
    pub(crate) name: Option<String>,
    pub(crate) number: Option<usize>,
    pub(crate) force: bool,
    pub(crate) no_body: bool,
}

impl TryFrom<&ArgMatches> for RepeaterExecSettings {
    type Error = CrusterCLIError;
    fn try_from(args: &ArgMatches) -> Result<Self, Self::Error> {
        let mut settings = RepeaterExecSettings {
            name: None,
            number: None,
            force: false,
            no_body: false,
        };

        let mark = args.get_one::<String>("mark").unwrap().to_string();
        if let Ok(number) = mark.parse::<usize>() {
            settings.number = Some(number);
        } else {
            settings.name = Some(mark);
        }

        settings.force = args.get_flag("force");

        if settings.name.is_none() && settings.number.is_none() {
            return Err(CrusterCLIError::from(
                "Use must specify number or name of repeater to work with",
            ));
        }

        settings.no_body = args.get_flag("no-body");

        return Ok(settings);
    }
}

fn open_editor(editor: &str, request: String) -> Result<String, CrusterCLIError> {
    let tmp_path = std::env::temp_dir().join(format!(
        "cruster-repeater-{}.txt",
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect::<String>()
    ));

    let mut fout = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(&tmp_path)?;

    let _ = fout.write(request.as_bytes())?;
    drop(fout);

    let _status = process::Command::new(editor).arg(&tmp_path).status()?;
    std::thread::sleep(std::time::Duration::from_millis(500));

    let mut fin = std::fs::File::open(&tmp_path)?;
    let mut edited_request = String::with_capacity(request.len() + 100);
    fin.read_to_string(&mut edited_request)?;
    drop(fin);

    std::fs::remove_file(tmp_path)?;

    // TODO: Make some toggle or warning
    if edited_request.ends_with("\n") {
        edited_request = edited_request.trim_end().to_string();
    }

    return Ok(edited_request);
}

fn get_ready_request(
    repeater: &mut RepeaterState,
    editor: &str,
    force: bool,
) -> Result<Request, CrusterCLIError> {
    if force {
        return Ok(repeater.make_reqwest()?);
    } else {
        repeater.request = open_editor(editor, repeater.request.clone())?;
        return Ok(repeater.make_reqwest()?);
    };
}

async fn handle_repeater(
    mut repeater: &mut RepeaterState,
    number: usize,
    path: &str,
    editor: &str,
    settings: &RepeaterExecSettings,
) -> Result<(), CrusterCLIError> {
    // TODO: update repeater after changing content-length
    let request = get_ready_request(&mut repeater, editor, settings.force)?;

    if settings.no_body {
        println!("\n{}\n", super::trim_body(repeater.request.as_str()));
    } else {
        println!("\n{}\n", repeater.request.as_str());
    }

    print!("{}\r", "Sending...");
    std::io::stdout().flush()?;

    let request_wrapper =
        crate::cruster_proxy::request_response::HyperRequestWrapper::from_reqwest(request).await?;
    let mut record = crate::siv_ui::repeater::RepeaterStateSerializable::from(&*repeater);
    let (wrapper, redirects_exceeded) = super::operations::execute_repeater(
        std::path::Path::new(path),
        &mut record,
        number,
        &request_wrapper,
        None,
    )
    .await?;
    let response_str = wrapper.to_string();
    repeater.response = TextContent::new(response_str.clone());

    if redirects_exceeded {
        eprintln!("REDIRECTS COUNT EXCEEDED\n");
    }

    if settings.no_body {
        println!("{}\n", super::trim_body(&response_str));
    } else {
        println!("{}\n", response_str);
    }

    return Ok(());
}

pub(crate) async fn execute(
    settings: &RepeaterExecSettings,
    path: &str,
    editor: &str,
) -> Result<(), CrusterCLIError> {
    let (index, records) =
        super::operations::load_selected(path, settings.number, settings.name.as_deref())?;
    let mut repeater = RepeaterState::try_from(records[index].clone())?;
    handle_repeater(&mut repeater, index, path, editor, settings).await
}
