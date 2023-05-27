pub(crate) mod request_response;
pub(super) mod events;

use request_response::{
    HyperRequestWrapper,
    HyperResponseWrapper
};
use log::debug;
use hudsucker::{
    async_trait::async_trait,
    hyper::{Body, Request, Response},
    tokio_tungstenite::tungstenite::Message,
    HttpHandler,
    HttpContext,
    WebSocketHandler,
    WebSocketContext,
    RequestOrResponse,
};
use std::{
    net::SocketAddr,
    hash::{Hash, Hasher},
    collections::hash_map::DefaultHasher
};

use std::time::SystemTime;
use cursive::{Cursive, CbSink};
use crossbeam_channel::Sender as CrossbeamSender;

use crate::CrusterError;
use super::siv_ui::put_proxy_data_to_storage;
use http::{Method, HeaderValue};
use events::ProxyEvents;

fn get_http_request_hash(client_addr: &SocketAddr, uri: &str, method: &str) -> usize {
    let mut hasher = DefaultHasher::new();
    client_addr.hash(&mut hasher);
    uri.hash(&mut hasher);
    method.hash(&mut hasher);
    SystemTime::now().hash(&mut hasher);

    let result = hasher.finish() as usize;

    return result
}

#[derive(Clone)]
pub(crate) struct CrusterHandler {
    pub(crate) proxy_tx: CrossbeamSender<ProxyEvents>,
    pub(crate) dump: bool,
    pub(crate) request_hash: usize,
    pub(crate) cursive_sink: CbSink,
}

#[derive(Clone)]
pub(crate) struct CrusterWSHandler {
    pub(crate) proxy_tx: CrossbeamSender<ProxyEvents>
}

#[async_trait]
impl HttpHandler for CrusterHandler {
    async fn handle_request(&mut self, _ctx: &HttpContext, req: Request<Body> ) -> RequestOrResponse {
        if req.method() == Method::CONNECT {
            return RequestOrResponse::Request(req);
        }

        return match HyperRequestWrapper::from_hyper(req).await {
            Ok((mut wrapper, new_req)) => {
                if ! wrapper.headers.contains_key("host") {
                    let host = wrapper.get_host();
                    let hv = HeaderValue::from_str(&host);
                    match hv {
                        Ok(hv) => {
                            wrapper.headers.insert("host", hv);
                        },
                        Err(err) => {
                            return self.send_error_message_from_request(err.into()).await;
                        }
                    }
                    
                }

                self.request_hash = get_http_request_hash(&_ctx.client_addr, &wrapper.uri, &wrapper.method);
                debug!("HTTP Request with id {}", &self.request_hash);
                match self.send_request_to_storage(wrapper).await {
                    Some(ror) => ror,
                    None => RequestOrResponse::Request(new_req)
                }
            },
            Err(err) => {
                self.send_error_message_from_request(err).await
            }
        }
    }

    async fn handle_response(&mut self, _ctx: &HttpContext, res: Response<Body>) -> Response<Body> {
        debug!("HTTP Response with id {}", &self.request_hash);
        return match HyperResponseWrapper::from_hyper(res).await {
            Ok((wrapper, new_res)) => {
                match self.send_response_to_storage(wrapper).await {
                    Some(response) => response,
                    None => new_res
                }
            },
            Err(err) => {
                self.send_error_message_from_response(err, self.request_hash).await
            }
        };
    }
}

impl CrusterHandler {
    fn make_blank_request(&self) -> hyper::Request<Body> {
        let request: Request<Body> = hyper::Request::default();
        return request;
    }

    async fn send_response_to_storage(&self, wrapper: HyperResponseWrapper) -> Option<hyper::Response<Body>> {
        let send_response_result = self.proxy_tx
            .send(ProxyEvents::ResponseSent((wrapper, self.request_hash)));

        if !self.dump {
            self.cursive_sink.send(
                Box::new(
                    |siv: &mut Cursive| {
                        put_proxy_data_to_storage(siv);
                    }
                )
            ).expect("FATAL: proxy could not sync with ui, while sending response!");
        }

        return match send_response_result {
            Ok(_) => { None },
            Err(e) => Some(
                self.send_error_message_from_response(e.into(), self.request_hash).await
            )
        };
    }

    async fn send_request_to_storage(&self, wrapper: HyperRequestWrapper) -> Option<RequestOrResponse> {
        let request_send_result = self.proxy_tx
            .send(ProxyEvents::RequestSent((wrapper, self.request_hash)));

        if !self.dump {
            self.cursive_sink.send(
                Box::new(
                    |siv: &mut Cursive| {
                        put_proxy_data_to_storage(siv);
                    }
                )
            ).expect("FATAL: proxy could not sync with ui, while sending request!");
        }

        match request_send_result {
            Ok(_) => { None },
            Err(err) => {
                let cerr = CrusterError::from(err);
                Some(self.send_error_message_from_request(cerr).await)
            }
        }
    }

    async fn send_error_message_from_request(&self, err: CrusterError) -> RequestOrResponse {
        let err_send_result = self.proxy_tx.send(ProxyEvents::Error((err, None)));
        match err_send_result {
            Ok(_) => return RequestOrResponse::Request(self.make_blank_request()),
            Err(send_err) => panic!("FATAL: cannot communicate between threads: {}", send_err)
        }
    }

    async fn send_error_message_from_response(&self, err: CrusterError, hash: usize) -> hyper::Response<Body> {
        let err_send_result = self.proxy_tx.send(ProxyEvents::Error((err, Some(hash))));
        match err_send_result {
            Ok(_) => return hyper::Response::new(hyper::Body::empty()),
            Err(send_err) => panic!("FATAL: cannot communicate with UI thread: {}", send_err)
        }
    }
}

// ---------------------------------------------------------------------------------------------- //

#[async_trait]
impl WebSocketHandler for CrusterWSHandler {
    async fn handle_message(&mut self, _ctx: &WebSocketContext, msg: Message) -> Option<Message> {
        let res = self.proxy_tx.send(
            ProxyEvents::WebSocketMessageSent((_ctx.clone(), msg.clone()))
        );

        if let Err(res) = res {
            panic!("FATAL! Could not send WS message over crossbeam channel: {}", res);
        }

        Some(msg)
    }
}
