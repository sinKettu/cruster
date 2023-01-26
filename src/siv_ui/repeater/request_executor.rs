use tokio::runtime::Runtime;
use std::{thread, str::FromStr};
use hyper::{Body, Client, Version, client::HttpConnector};
use cursive::{Cursive, utils::span::SpannedString, theme::Style};
use http::{Response, HeaderMap, Request, header::HeaderName, HeaderValue};

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

async fn execute_request(req: hyper::Request<Body>) -> Result<Response<Body>, CrusterError> {
    let scheme = req.uri().scheme().unwrap().as_str();
    let sending_result = if scheme.starts_with("https") {
        let tls = hyper_rustls::HttpsConnectorBuilder::new()
            .with_webpki_roots()
            .https_or_http()
            .enable_http1();

        let tls = tls.build();
        // there is an issue with HTTP/2 in Hyper: https://github.com/hyperium/hyper/issues/2417
        if req.version() == Version::HTTP_2 {
            let mut client_builder = Client::builder();
            client_builder.http2_only(true);
            let client = client_builder.build::<_, hyper::Body>(tls);
            client.request(req).await
        }
        else {
            let client = Client::builder().build::<_, hyper::Body>(tls);
            client.request(req).await
        }
    }
    else {
        if req.version() == Version::HTTP_2 {
            let mut builder = Client::builder();
            builder.http2_only(true);
            let client = builder.build::<_, hyper::Body>(HttpConnector::new());
            
            client.request(req).await
        }
        else {
            let client = Client::new();
            client.request(req).await
        }
    };

    return match sending_result {
        Ok(rsp) => {
           Ok(rsp)
        },
        Err(e) => {
            Err(CrusterError::from(e))
        }
    };
}

async fn prepare_for_redirect(
    rsp_headers: &HeaderMap,
    saved_headers: &HeaderMap,
    saved_version: &Version
) -> Result<Option<Request<Body>>, CrusterError> {

    match rsp_headers.get("location") {
        Some(location) => {
            let uri = location.to_str()?;
            let first_line_splitted: Vec<&str> = uri.split('/').collect();
            let host = first_line_splitted[2].clone();

            let mut request_builder = hyper::Request::builder()
                .method("GET")
                .version(saved_version.clone())
                .uri(uri);
            
            let possible_error_message = "Could not build request in repeater after reaching redirect".to_string();
            for (k, v) in saved_headers.into_iter() {
                request_builder
                    .headers_mut()
                    .ok_or(CrusterError::HyperRequestBuildingError(possible_error_message.clone()))?
                    .insert(k.clone(), v.clone());
            }

            request_builder
                .headers_mut()
                .ok_or(CrusterError::HyperRequestBuildingError(possible_error_message.clone()))?
                .insert(
                    HeaderName::from_str("Host").unwrap(),
                    HeaderValue::from_str(host).unwrap()
                );
            
            let request = request_builder.body(Body::empty())?;
            return Ok(Some(request));
        },
        None => {
            return Ok(None);
        }
    }
}

async fn handle_sending(
    req: hyper::Request<Body>,
    state_index: usize,
    need_redirect: bool,
    sink: cursive::CbSink
) -> Result<SpannedString<Style>, CrusterError> {

    let mut actual_request = req;
    let saved_headers = actual_request.headers().clone();
    let saved_version = actual_request.version();
    let mut redirect_counter: u8 = 0;
    loop {
        let rsp = execute_request(actual_request).await?;
        if ! need_redirect {
            let wrapper = HyperResponseWrapper::from_hyper(rsp).await?;
            let rsp_text = req_res_spanned::response_wrapper_to_spanned(&wrapper.0);
            return Ok(rsp_text);
        }

        let rsp_headers = rsp.headers();
        match prepare_for_redirect(rsp_headers, &saved_headers, &saved_version).await? {
            Some(redirect_request) => {
                if redirect_counter >= 10 {
                    let err = format!("Too many redirects reached in repeater #{}", state_index);
                    return Err(CrusterError::UndefinedError(err));
                }

                let (wrapper, request) = HyperRequestWrapper::from_hyper(redirect_request).await?;
                sink.send(
                    Box::new(
                        move |siv: &mut Cursive| {
                            super::handle_repeater_event(
                                siv,
                                state_index,
                                RepeaterEvent::RequestChanged(wrapper.to_string())
                            )
                        }
                    )
                ).expect("FATAL: Could not synchronize threads while repeating (RequestChanged case).");

                actual_request = request;
                redirect_counter += 1;
            },
            None => {
                let wrapper = HyperResponseWrapper::from_hyper(rsp).await?;
                let rsp_text = req_res_spanned::response_wrapper_to_spanned(&wrapper.0);
                return Ok(rsp_text);
            }
        }
    }
}

pub(super) fn send_request_detached(req: hyper::Request<Body>, state_index: usize, redirects: bool, sink: cursive::CbSink) {
    let _thrd = thread::spawn(
        move || {
            let runtime = Runtime::new().unwrap();
            match runtime.block_on(handle_sending(req, state_index, redirects, sink.clone())) {
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
