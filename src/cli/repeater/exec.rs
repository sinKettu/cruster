use std::io::Write;
use std::{process, io::Read};
use rand::{distributions::Alphanumeric, Rng};

use clap::ArgMatches;
use cursive::views::TextContent;
use http::{HeaderMap, HeaderValue};
use reqwest::{self, Request, Response};

use super::RepeaterIterator;
use crate::cli::CrusterCLIError;
use crate::siv_ui::repeater::{RepeaterState, RepeaterParameters};


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
            no_body: false
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

        settings.no_body = args.get_flag("no-body");

        return Ok(settings);
    }
}

async fn follow_redirect(response: Response, redirects: &mut usize, cookie: Option<&HeaderValue>) -> Result<Response, CrusterCLIError> {
    let client = reqwest::Client::builder()
        .use_rustls_tls()
        .redirect(reqwest::redirect::Policy::none())
        .http1_only()
        .build()?;

    let mut current_response = response;

    while redirects > &mut 0 && current_response.status().is_redirection() {
        let location_str = current_response.headers().get("location");
        if let None = location_str {
            return Err(
                CrusterCLIError::from("Found redirection, but could not find 'Location' header in response")
            );
        }

        let location_str = location_str.unwrap().to_str()?;
        let location_url = reqwest::Url::parse(location_str)?;

        let mut headers = HeaderMap::new();
        headers.insert("referer", HeaderValue::from_str(current_response.url().as_str())?);
        if let Some(cookie) = cookie {
            headers.insert("cookie", cookie.clone());
        }

        current_response = client.request(reqwest::Method::GET, location_url)
            .headers(headers)
            .send()
            .await?;

        *redirects = *redirects - 1;
    }

    Ok(current_response)
}

async fn send_request(request: Request, params: &RepeaterParameters) -> Result<(Response, usize), CrusterCLIError> {
    let cookie = if let Some(cookie) = request.headers().get("cookie") {
        Some(cookie.to_owned())
    }
    else {
        None
    };

    let client = reqwest::Client::builder()
        .use_rustls_tls()
        .redirect(reqwest::redirect::Policy::none())
        .http1_only()
        .build()?;

    let response = client.execute(request).await?;

    if response.status().is_redirection() && params.redirects {
        let mut counter = params.max_redirects.saturating_sub(1);
        let response = follow_redirect(response, &mut counter, cookie.as_ref()).await?;
        return Ok((response, counter));
    }

    return Ok((response, params.max_redirects));
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
    std::thread::sleep(std::time::Duration::from_millis(500));

    let mut fin = std::fs::File::open(&tmp_path)?;
    let mut edited_request = String::with_capacity(request.len() + 100);
    fin.read_to_string(&mut edited_request)?;

    std::fs::remove_file(tmp_path)?;

    // TODO: Make some toggle or warning
    if edited_request.ends_with("\n") {
        edited_request = edited_request.trim_end().to_string();
    }

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

async fn handle_repeater(mut repeater: &mut RepeaterState, number: usize, path: &str, editor: &str, settings: &RepeaterExecSettings) -> Result<(), CrusterCLIError> {
    // TODO: update repeater after changing content-length
    let request = get_ready_request(&mut repeater, editor, settings.force)?;

    if settings.no_body {
        println!("\n{}\n", super::trim_body(repeater.request.as_str()));
    } else {
        println!("\n{}\n", repeater.request.as_str());
    }
    
    print!("{}\r", "Sending...");
    std::io::stdout().flush()?;
    
    super::update_repeaters(path, &repeater, number.to_owned())?;

    let (response, redirects) = send_request(request, &repeater.parameters).await?;
    let wrapper = crate::cruster_proxy::request_response::HyperResponseWrapper::from_reqwest(response).await?;
    let response_str = wrapper.to_string();
    repeater.response = TextContent::new(response_str.clone());

    super::update_repeaters(path, &repeater, number.to_owned())?;

    if redirects == 0 {
        eprintln!("REDIRECTS COUNT EXCEEDED\n");
    }

    if settings.no_body {
        println!("{}\n", super::trim_body(&response_str));
    } else {
        println!("{}\n", response_str);
    }

    return Ok(())
}

pub(crate) async fn execute(settings: &RepeaterExecSettings, path: &str, editor: &str) -> Result<(), CrusterCLIError> {
    let repeater_iter = RepeaterIterator::new(path);
    for (i, mut repeater) in repeater_iter.enumerate() {
        if let Some(number) = settings.number.as_ref() {
            if &(i + 1) == number {
                return handle_repeater(&mut repeater, i, path, editor, settings).await;
            }

            continue;
        }

        if let Some(name) = settings.name.as_ref() {
            if &repeater.name == name {
                return handle_repeater(&mut repeater, i, path, editor, settings).await;
            }

            continue;
        }
    }

    Err(
        CrusterCLIError::from("Cannot find repeater by specified mark (number/name)")
    )
}