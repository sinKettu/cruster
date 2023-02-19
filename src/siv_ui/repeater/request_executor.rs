use reqwest;
use std::{thread, str::FromStr};
use tokio::runtime::Runtime;
use cursive::{Cursive, utils::span::SpannedString, theme::Style};

use crate::{
    utils::CrusterError,
    siv_ui::req_res_spanned,
    cruster_proxy::request_response::{HyperResponseWrapper, HyperRequestWrapper},
};
use http::{HeaderMap, HeaderValue};

pub(super) enum RepeaterEvent {
    Ready(SpannedString<Style>),
    Error(CrusterError),
    RequestChanged(String),
}

fn send_ready_event(txt: SpannedString<Style>, state_index: usize, sink: cursive::CbSink) {
    sink.send(
        Box::new(
            move |siv: &mut Cursive| {
                super::handle_repeater_event(
                    siv,
                    state_index,
                    RepeaterEvent::Ready(txt)
                )
            }
        )
    ).expect("FATAL: Could not synchronize threads while repeating (Ready case).");
}

fn send_error_event(err: CrusterError, state_index: usize, sink: cursive::CbSink) {
    sink.send(
        Box::new(
            move |siv: &mut Cursive| {
                super::handle_repeater_event(
                    siv,
                    state_index,
                    RepeaterEvent::Error(err)
                )
            }
        )
    ).expect("FATAL: Could not synchronize threads while repeating (Error case).");
}

fn send_request_changed_event(request: String, state_index: usize, sink: cursive::CbSink) {
    sink.send(
        Box::new(
            move |siv: &mut Cursive| {
                super::handle_repeater_event(
                    siv,
                    state_index,
                    RepeaterEvent::RequestChanged(request)
                )
            }
        )
    ).expect("FATAL: Could not synchronize threads while repeating (RequestChanged case).");
}

fn forge_request_for_redirect(client: &reqwest::Client, rsp_hdrs: &HeaderMap<HeaderValue>, req_hdrs: &mut HeaderMap<HeaderValue>, prev_url: &reqwest::Url) -> Result<reqwest::Request, CrusterError> {
    // Get Location header from response or handle error
    let location = match rsp_hdrs.get("location") {
        Some(location_str) => {
            let possible_location = reqwest::Url::from_str(location_str.to_str()?);
            if let Err(_) = possible_location {
                return Err(CrusterError::UndefinedError("Could not follow redirect because could not parse URL from 'Location'".to_string()));
            }

            possible_location.unwrap()
        },
        None => {
            return Err(CrusterError::UndefinedError("Could not follow redirect because did not found 'Location'".to_string()));
        }
    };

    // Define new Host header for request (use from Location or previous one)
    let prev_host = req_hdrs.get("host").unwrap().to_str()?;
    let next_host = location
        .host_str()
        .unwrap_or(prev_host)
        .to_string();

    // Try to remove, because inserting does not rewrite existing
    req_hdrs.remove("host");
    req_hdrs.remove("referer");

    let request = client.request(reqwest::Method::GET, location)
        .headers(req_hdrs.to_owned())
        .header("Host", next_host)
        .header("Referer", prev_url.to_string())
        .build()?;

    return Ok(request);
}

async fn send_reqwest(req: reqwest::Request, state_index: usize, redirects: bool, sink: cursive::CbSink) -> Result<SpannedString<Style>, CrusterError> {
    let client = reqwest::ClientBuilder::new()
        .http1_only()
        .use_rustls_tls()
        .redirect(reqwest::redirect::Policy::none());

    let client = client.build()?;
    let mut headers_backup = req.headers().clone();
    let mut url_backup = req.url().clone();
    let mut request = req;
    let mut redirect_count: u8 = 0;

    loop {
        let rsp = client.execute(request).await?;
        // Manual redirects implementation because reqwest does not change Host header and always reaches maximum count
        if rsp.status().is_redirection() && redirects {
            if redirect_count >= 10 {
                return Err(CrusterError::UndefinedError("Redirects max count reached".to_string()));
            }

            redirect_count += 1;
            
            request = forge_request_for_redirect(
                &client,
                rsp.headers(),
                &mut headers_backup,
                &url_backup
            )?;

            headers_backup = request.headers().clone();
            url_backup = request.url().clone();

            let str_request = HyperRequestWrapper::from_reqwest(request.try_clone().unwrap())
                .await?
                .to_string();

            send_request_changed_event(str_request, state_index, sink.clone());
        }
        else {
            let wrapper = HyperResponseWrapper::from_reqwest(rsp).await?;
            let styled_text = req_res_spanned::response_to_spanned_full(&wrapper);
            return Ok(styled_text);
        }
    }
}

pub(super) fn send_request_detached(req: reqwest::Request, state_index: usize, redirects: bool, sink: cursive::CbSink) {
    let _thrd = thread::spawn(
        move || {
            let runtime = Runtime::new().unwrap();
            match runtime.block_on(send_reqwest(req, state_index, redirects, sink.clone())) {
                Ok(response_text) => send_ready_event(response_text, state_index, sink),
                Err(e) => send_error_event(e, state_index, sink)
            }
        }
    );
}
