pub(crate) mod request_response;

use std::collections::HashSet;
use std::net::SocketAddr;
use std::time::Duration;
use request_response::{CrusterWrapper, HyperRequestWrapper, HyperResponseWrapper};
use tokio::sync::mpsc::Sender;
use hudsucker::{async_trait::async_trait, hyper::{Body, Request, Response}, HttpHandler, RequestOrResponse, HttpContext, MessageHandler, MessageContext};
use hudsucker::tungstenite::Message;
use hudsucker::tungstenite::protocol::WebSocketContext;
use log::debug;
use rand as rnd;
use rand::Rng;
use tokio::time::sleep;
use tokio_tungstenite::tungstenite::protocol::WebSocketConfig;

#[derive(Clone)]
pub(crate) struct CrusterHandler {
    pub(crate) proxy_tx: Sender<(CrusterWrapper, HttpContext)>
}

#[derive(Clone)]
pub(crate) struct CrusterWSHandler;

#[async_trait]
impl HttpHandler for CrusterHandler {
    async fn handle_request(
        &mut self,
        _ctx: &HttpContext,
        req: Request<Body>
    ) -> RequestOrResponse
    {
        let (wrapper, new_req) = HyperRequestWrapper::from_hyper(req).await;
        debug!("REQUEST -- {} -- {}", _ctx.client_addr.to_string(), wrapper.uri.as_str());
        // TODO: handle error in a better way
        match self.proxy_tx.send((CrusterWrapper::Request(wrapper), _ctx.clone())).await {
            Ok(_) => RequestOrResponse::Request(new_req),
            Err(e) => panic!("Could not send: {}", e)
        }
    }

    async fn handle_response(&mut self, _ctx: &HttpContext, res: Response<Body>) -> Response<Body> {
        let (wrapper, new_res) = HyperResponseWrapper::from_hyper(res).await;
        let a = String::from_utf8_lossy(wrapper.body.as_slice()).to_string();
        debug!("RESPONSE -- {} -- {}", _ctx.client_addr.to_string(), &a[..22]);
        match self.proxy_tx.send((CrusterWrapper::Response(wrapper), _ctx.clone())).await {
            Ok(_) => {
                new_res
            },
            Err(e) => panic!("Could not send to thread: {}", e)
        }
    }
}

// ---------------------------------------------------------------------------------------------- //

#[async_trait]
impl MessageHandler for CrusterWSHandler {
    async fn handle_message(&mut self, _ctx: &MessageContext, msg: Message) -> Option<Message> {
        println!("{:?}", msg);
        Some(msg)
    }
}
