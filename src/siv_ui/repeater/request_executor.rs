///
/// This module turned out to be not very pleasant and convenient. I would like to rewrite it in the future.
/// 

use http::Response;
use bstr::ByteSlice;
use hyper::{Body, Client};
use tokio::runtime::Runtime;
use std::{time::Instant, thread};
use cursive::{Cursive, views::TextView};

use super::RepeaterRequestHandler;
use crate::{
    utils::CrusterError,
    cruster_proxy::request_response::HyperResponseWrapper,
    siv_ui::{sivuserdata::SivUserData, req_res_spanned::response_wrapper_to_spanned},
};

async fn execute_request(req: hyper::Request<Body>, state_index: usize, sink: cursive::CbSink) {
    let scheme = req.uri().scheme().unwrap().as_str();
    let sending_result = if scheme.starts_with("https") {
        let tls = hyper_tls::HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(tls);
        client.request(req).await
    }
    else {
        let client = Client::new();
        client.request(req).await
    };

    match sending_result {
        Ok(rsp) => {
            // On this moment we should check for redirect
            let conversion_result = HyperResponseWrapper::from_hyper(rsp).await;
            match conversion_result {
                Ok((wrapper, _)) => {
                    // TODO: send with sink
                    let spanned = response_wrapper_to_spanned(&wrapper);
                },
                Err(e) => {
                    // TODO: send with sink
                    let _err = CrusterError::from(e);
                }
            }
        },
        Err(e) => {
            // TODO: send with sink
            let _err = CrusterError::from(e);
        }
    }
}

pub(super) fn send_request_detached(req: hyper::Request<Body>, state_index: usize, sink: cursive::CbSink) {
    let _thrd = thread::spawn(
        move || {
            let runtime = Runtime::new().unwrap();
            runtime.block_on(execute_request(req, state_index, sink))
        }
    );
}

/// This routine sending request using `Hyper` lib. Instead of blocking on sending, it creates thread
/// and push this to cursive's callback, that checks if thread is finished. Such trick helps UI to
/// process other events while waiting for sending is completed.
pub(super) fn send_hyper_request(siv: &mut Cursive, req: hyper::Request<Body>, beginning: Instant, idx: usize) {
    let scheme = req.uri().scheme().unwrap().as_str();
    let send_result = if scheme.starts_with("https") {
        let tls = hyper_tls::HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(tls);

        thread::spawn(move || {
            let runtime = Runtime::new().unwrap();
            let rsp = runtime.block_on(client.request(req));
            return rsp;
        })
    }
    else {
        let client = Client::new();
        thread::spawn(move || {
            let runtime = Runtime::new().unwrap();
            let rsp = runtime.block_on(client.request(req));
            return rsp;
        })
    };

    siv.cb_sink().send(
        Box::new(
            move |s: &mut Cursive| { wait_for_response(s, send_result, beginning, idx); }
        )
    ).expect("Could not await for request is sent from repeater!");
}

/// Routine takes handler for a thread sending request and checks if it is finished. When sending is finished
/// routine checks if response is redirect, in this case it will call `follow_redirect` routine.
fn wait_for_response(siv: &mut Cursive, handler: RepeaterRequestHandler, beginning: Instant, idx: usize) {
    if handler.is_finished() {
        let send_result = handler.join().unwrap();

        match send_result {
            Ok(rsp) => {
                let ud: &mut SivUserData = siv.user_data().unwrap();
                let repeater_state = &mut ud.repeater_state[idx];
                if repeater_state.parameters.redirects {
                    let duration = Instant::now().duration_since(beginning).as_secs();
                    ud.status.set_message(format!("Request sending: {} sec", duration));

                    siv.cb_sink().send(
                        Box::new(
                            move |s: &mut Cursive| { follow_redirect(s, rsp, beginning, idx); }
                        )
                    ).expect("Could not await for request is sent from repeater!");
                }
                else {
                    let ud: &mut SivUserData = siv.user_data().unwrap();
                    ud.status.set_message(format!("Repeater with index {} is finished!", idx));

                    siv.cb_sink().send(
                        Box::new(
                            move |s: &mut Cursive| { hyper_response_to_view_content(s, rsp, idx); }
                        )
                    ).expect("Could not await for request is sent from repeater!");                    
                }
            },
            Err(e) => {
                let ud: &mut SivUserData = siv.user_data().unwrap();
                let err = CrusterError::UndefinedError(
                    format!("Error while sending request in repeater: {}", e.to_string())
                );

                ud.status.set_message(format!("Error occured in repeater #{}", idx));
                ud.push_error(err);
            }
        }
    }
    else {
        let duration = Instant::now().duration_since(beginning).as_secs();
        let ud: &mut SivUserData = siv.user_data().unwrap();
        ud.status.set_message(format!("Request sending: {} sec", duration));
        
        siv.cb_sink().send(
            Box::new(
                move |s: &mut Cursive| { wait_for_response(s, handler, beginning, idx); }
            )
        ).expect("Could not await for request is sent from repeater!");
    }
}

/// Routine builds new request to follow redirect and checks redirects depth to avoid infinit redirects
fn follow_redirect(siv: &mut Cursive, rsp: Response<Body>, beginning: Instant, idx: usize) {
    let ud: &mut SivUserData = siv.user_data().unwrap();
    let repeater_state = &mut ud.repeater_state[idx];

    if repeater_state.redirects_reached == repeater_state.parameters.max_redirects {
        ud.status.set_message("Maximum number of redirects reached");
        siv.cb_sink().send(
            Box::new(
                move |s: &mut Cursive| { hyper_response_to_view_content(s, rsp, idx); }
            )
        ).expect("Could not await for request is sent from repeater!");

        return;
    }

    repeater_state.redirects_reached += 1;

    if rsp.status().is_redirection() {
        let next_uri = rsp.headers().get("location");
        if let Some(next_uri) = next_uri {
            let next_uri = next_uri.as_bytes().to_str_lossy().to_string();
            match repeater_state.make_request_to_redirect(&next_uri) {
                Ok(request) => {
                    siv.cb_sink().send(
                        Box::new(
                            move |s: &mut Cursive| {
                                let ud: &mut SivUserData = s.user_data().unwrap();
                                let repeater_state = &mut ud.repeater_state[idx];
                                // Very not effective, should be rewritten
                                let request = repeater_state.request.clone();
                                
                                let _ = s.call_on_name("repeater-request-static", move |t: &mut TextView| {
                                    t.set_content(request);
                                });
                            }
                        )
                    ).expect("Could not await for request is sent from repeater!");              

                    siv.cb_sink().send(
                        Box::new(
                            move |s: &mut Cursive| { send_hyper_request(s, request, beginning, idx); }
                        )
                    ).expect("Could not await for request is sent from repeater!");
                },
                Err(err) => {
                    ud.status.set_message(format!("Error while trying to follow redirect in repeater: {}", err.to_string()));
                    ud.push_error(err);
                }
            }   
        }
    }
    else {
        ud.status.set_message(format!("Repeater with index {} is finished!", idx));
        siv.cb_sink().send(
            Box::new(
                move |s: &mut Cursive| { hyper_response_to_view_content(s, rsp, idx); }
            )
        ).expect("Could not await for request is sent from repeater!");
    }
}

/// Routine converts `Response` structure from `Hyper` lib to printable format for UI
fn hyper_response_to_view_content(siv: &mut Cursive, rsp: Response<Body>, idx: usize) {
    let ud: &mut SivUserData = siv.user_data().unwrap();
    let repeater_state = &mut ud.repeater_state[idx];

    let possible_wrapper = thread::spawn(move || {
        let runtime = Runtime::new().unwrap();
        let wrapper = runtime.block_on(HyperResponseWrapper::from_hyper(rsp));
        return wrapper;
    }).join().unwrap();

    if let Err(err) = possible_wrapper {
        ud.push_error(err);
    }
    else {
        let wrapper = possible_wrapper.unwrap().0;
        repeater_state.response.set_content(response_wrapper_to_spanned(&wrapper));
    }
}
