pub(crate) mod request_response;

use bstr::ByteSlice;
use request_response::{CrusterWrapper, HyperRequestWrapper, HyperResponseWrapper};
use tokio::sync::mpsc::Sender;
use log::debug;
use hudsucker::{
    async_trait::async_trait,
    hyper::{Body, Request, Response},
    tungstenite::Message,
    HttpHandler,
    RequestOrResponse,
    HttpContext,
    MessageHandler,
    MessageContext
};
use std::{
    cmp::min,
    hash::{Hash, Hasher},
    net::SocketAddr,
    collections::hash_map::DefaultHasher
};
use std::borrow::Borrow;
use std::io::Empty;
use tokio::sync::mpsc::error::SendError;
use crate::CrusterError;

fn get_http_request_hash(client_addr: &SocketAddr, uri: &str, method: &str) -> usize {
    let mut hasher = DefaultHasher::new();
    client_addr.hash(&mut hasher);
    uri.hash(&mut hasher);
    method.hash(&mut hasher);
    let result = hasher.finish() as usize;

    return result
}

#[derive(Clone)]
pub(crate) struct CrusterHandler {
    pub(crate) proxy_tx: Sender<(CrusterWrapper, usize)>,
    pub(crate) err_tx: Sender<CrusterError>,
    pub(crate) dump: bool,
    pub(crate) request_hash: usize
}

#[derive(Clone)]
pub(crate) struct CrusterWSHandler {
    pub(crate) dump: bool,
    pub(crate) from_client: bool
}

#[async_trait]
impl HttpHandler for CrusterHandler {
    async fn handle_request(&mut self, _ctx: &HttpContext, req: Request<Body> ) -> RequestOrResponse {
        if self.dump {
            let (parts, body) = req.into_parts();
            println!("http ==> {} {}", parts.method.clone().to_string(), parts.uri.clone().to_string());
            for (k, v) in &parts.headers {
                let v_str: &str = match v.to_str() {
                    Ok(s) => s,
                    // TODO: something better
                    Err(_) => return RequestOrResponse::Request(Request::from_parts(parts, body))
                };

                println!("http ==> {}: {}", k.as_str(), v_str);
            }
            let body = match hyper::body::to_bytes(body).await {
                Ok(body_bytes) => {
                    body_bytes
                },
                Err(err) => {
                    println!("http ==> [ERROR] ~Faced with troubles to handle request body~ [ERROR");
                    println!("http ==>");
                    return RequestOrResponse::Request(self.make_blank_request());
                }
            };
            let cloned_body = body.clone().to_vec();
            let cloned_body = body[..{min(22, cloned_body.len())}].to_str_lossy().to_string();
            println!("http ==> {}", cloned_body);

            println!("http ==>");
            RequestOrResponse::Request(Request::from_parts(parts, Body::from(body)))
        }
        else {
            return match HyperRequestWrapper::from_hyper(req).await {
                Ok((wrapper, new_req)) => {
                    self.request_hash = get_http_request_hash(&_ctx.client_addr, &wrapper.uri, &wrapper.method);
                    debug!("- CRUSTER - HTTP Request with id {}", &self.request_hash);
                    let request_send_result = self.proxy_tx
                        .send((CrusterWrapper::Request(wrapper), self.request_hash))
                        .await;

                    match request_send_result {
                        Ok(_) => {
                            RequestOrResponse::Request(new_req)
                        },
                        Err(e) => {
                            self.send_error_message_from_request(e.into()).await
                        }
                    }
                },
                Err(err) => {
                    self.send_error_message_from_request(err).await
                }
            }
        }
    }

    async fn handle_response(&mut self, _ctx: &HttpContext, res: Response<Body>) -> Response<Body> {
        if self.dump {
            let (parts, body) = res.into_parts();
            println!("http <== {}", parts.status.as_str());
            for (k, v) in &parts.headers {
                let v_str = match v.to_str() {
                    Ok(s) => s,
                    Err(_) => return Response::from_parts(parts, body)
                };

                println!("http <== {}: {}", k.as_str(), v_str);
            }
            println!("http <==");
            let body = match hyper::body::to_bytes(body).await {
                Ok(bytes) => bytes,
                Err(e) => {
                    println!("http <== [ERROR] ~Faced with troubles to handle response body~ [ERROR");
                    println!("http <==");
                    return hyper::Response::new(hyper::Body::empty());
                }
            };
            let body_clone = body.clone();
            let body_clone = body[..{min(body_clone.len(), 22)}].to_str_lossy().to_string();
            println!("http <== {}", body_clone);

            println!("http <==");
            return Response::from_parts(parts, Body::from(body));
        }
        else {
            debug!("- CRUSTER - HTTP Response with id {}", &self.request_hash);
            let body_parsing_result = HyperResponseWrapper::from_hyper(
                res,
                Some(self.err_tx.borrow())
            ).await;

            return match body_parsing_result {
                Ok((wrapper, new_res)) => {
                    let send_response_result = self.proxy_tx
                        .send((CrusterWrapper::Response(wrapper), self.request_hash))
                        .await;

                    match send_response_result {
                        Ok(_) => { new_res },
                        Err(e) => self.send_error_message_from_response(e.into()).await
                    }
                },
                Err(err) => self.send_error_message_from_response(err).await
            }

        }
    }
}

impl CrusterHandler {
    fn make_blank_request(&self) -> hyper::Request<Body> {
        let request: Request<Body> = hyper::Request::default();
        return request;
    }

    async fn send_error_message_from_request(&self, err: CrusterError) -> RequestOrResponse {
        let err_send_result = self.err_tx
            .send(err)
            .await;

        match err_send_result {
            Ok(_) => return RequestOrResponse::Request(self.make_blank_request()),
            Err(send_err) => panic!("FATAL: cannot communicate with UI thread: {}", send_err)
        }
    }

    async fn send_error_message_from_response(&self, err: CrusterError) -> hyper::Response<Body> {
        let err_send_result = self.err_tx
            .send(err)
            .await;

        match err_send_result {
            Ok(_) => return hyper::Response::new(hyper::Body::empty()),
            Err(send_err) => panic!("FATAL: cannot communicate with UI thread: {}", send_err)
        }
    }
}

// ---------------------------------------------------------------------------------------------- //

#[async_trait]
impl MessageHandler for CrusterWSHandler {
    async fn handle_message(&mut self, _ctx: &MessageContext, msg: Message) -> Option<Message> {
        if self.dump {
            println!(
                "wskt {} {}, {}: {:?}",
                {if self.from_client { "==>" } else { "<==" }},
                _ctx.client_addr,
                _ctx.server_uri,
                &msg
            );
            Some(msg)
        }
        else {
            Some(msg)
        }
    }
}
