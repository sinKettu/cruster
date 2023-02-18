use reqwest;
use std::{thread, str::FromStr};
use tokio::runtime::Runtime;
use cursive::{Cursive, utils::span::SpannedString, theme::Style};

use crate::{
    utils::CrusterError,
    siv_ui::req_res_spanned,
    cruster_proxy::request_response::{HyperResponseWrapper, HyperRequestWrapper},
};

pub(super) enum RepeaterEvent {
    Ready(SpannedString<Style>),
    Error(CrusterError),
    RequestChanged(String),
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
            // Get Location header from response or handle error
            let location = match rsp.headers().get("location") {
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
            let prev_host = headers_backup.get("host").unwrap().to_str()?;
            let next_host = location
                .host_str()
                .unwrap_or(prev_host)
                .to_string();

            // Try to remove, because inserting does not rewrite existing
            headers_backup.remove("host");
            headers_backup.remove("referer");
            request = client.request(reqwest::Method::GET, location)
                .headers(headers_backup)
                .header("Host", next_host)
                .header("Referer", url_backup.clone().to_string())
                .build()?;

            headers_backup = request.headers().clone();
            url_backup = request.url().clone();

            let str_request = HyperRequestWrapper::from_reqwest(request.try_clone().unwrap())
                .await?
                .to_string();

            sink.send(
                Box::new(
                    move |siv: &mut Cursive| {
                        super::handle_repeater_event(
                            siv,
                            state_index,
                            RepeaterEvent::RequestChanged(str_request)
                        )
                    }
                )
            ).expect("FATAL: Could not synchronize threads while repeating (RequestChanged case).");

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
                Ok(response_text) => {
                    sink.send(
                        Box::new(
                            move |siv: &mut Cursive| {
                                super::handle_repeater_event(
                                    siv,
                                    state_index,
                                    RepeaterEvent::Ready(response_text)
                                )
                            }
                        )
                    ).expect("FATAL: Could not synchronize threads while repeating (Ready case).");
                },
                Err(e) => {
                    sink.send(
                        Box::new(
                            move |siv: &mut Cursive| {
                                super::handle_repeater_event(
                                    siv,
                                    state_index,
                                    RepeaterEvent::Error(e)
                                )
                            }
                        )
                    ).expect("FATAL: Could not synchronize threads while repeating (Error case).");
                }
            }
        }
    );
}
