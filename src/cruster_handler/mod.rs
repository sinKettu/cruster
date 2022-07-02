pub(crate) mod request_response;

use std::cmp::min;
use std::collections::HashSet;
use std::net::SocketAddr;
use std::time::Duration;
use bstr::ByteSlice;
use request_response::{CrusterWrapper, HyperRequestWrapper, HyperResponseWrapper};
use tokio::sync::mpsc::Sender;
use hudsucker::{async_trait::async_trait, hyper::{Body, Request, Response}, HttpHandler, RequestOrResponse, HttpContext, MessageHandler, MessageContext};
use hudsucker::tungstenite::Message;
use hudsucker::tungstenite::protocol::WebSocketContext;
use log::debug;
use rand as rnd;
use rand::Rng;
use serde_yaml::Value::String;
use tokio::time::sleep;
// use tokio_tungstenite::tungstenite::protocol::WebSocketConfig;

#[derive(Clone)]
pub(crate) struct CrusterHandler {
    pub(crate) proxy_tx: Sender<(CrusterWrapper, HttpContext)>,
    pub(crate) dump: bool
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
                    Err(e) => return RequestOrResponse::Request(Request::from_parts(parts, body))
                };

                println!("http ==> {}: {}", k.as_str(), v_str);
            }
            let body = hyper::body::to_bytes(body).await.unwrap();
            let cloned_body = body.clone().to_vec();
            let cloned_body = body[..{min(22, cloned_body.len())}].to_str_lossy().to_string();
            println!("http ==> {}", cloned_body);

            println!("http ==>");
            RequestOrResponse::Request(Request::from_parts(parts, Body::from(body)))
        }
        else {
            let (mut wrapper, new_req) = HyperRequestWrapper::from_hyper(req).await;
            // TODO: handle error in a better way
            match self.proxy_tx.send((CrusterWrapper::Request(wrapper), _ctx.clone())).await {
                Ok(_) => RequestOrResponse::Request(new_req),
                Err(e) => panic!("Could not send: {}", e)
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
                    Err(e) => return Response::from_parts(parts, body)
                };

                println!("http <== {}: {}", k.as_str(), v_str);
            }
            println!("http <==");
            let body = hyper::body::to_bytes(body).await.unwrap();
            let body_clone = body.clone();
            let body_clone = body[..{min(body_clone.len(), 22)}].to_str_lossy().to_string();
            println!("http <== {}", body_clone);

            println!("http <==");
            return Response::from_parts(parts, Body::from(body));
        }
        else {
            let (mut wrapper, new_res) = HyperResponseWrapper::from_hyper(res).await;
            match self.proxy_tx.send((CrusterWrapper::Response(wrapper), _ctx.clone())).await {
                Ok(_) => {
                    new_res
                },
                Err(e) => panic!("Could not send to thread: {}", e)
            }
        }
    }
}

// ---------------------------------------------------------------------------------------------- //

#[async_trait]
impl MessageHandler for CrusterWSHandler {
    async fn handle_message(&mut self, _ctx: &MessageContext, msg: Message) -> Option<Message> {
        if self.dump {
            println!("wskt {} {}, {}: {:?}", {if self.from_client { "==>" } else { "<==" }}, _ctx.client_addr, _ctx.server_uri, &msg);

            Some(msg)
        }
        else {
            // println!("{:?}", msg);
            Some(msg)
        }
    }
}
