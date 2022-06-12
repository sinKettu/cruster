pub(crate) mod request_response;

use request_response::{CrusterWrapper, HyperRequestWrapper};
use tokio::sync::mpsc::Sender;
use hudsucker::{
    async_trait::async_trait,
    hyper::{Body, Request, Response, self},
    HttpHandler,
    RequestOrResponse,
    HttpContext
};

#[derive(Clone)]
pub(crate) struct CrusterHandler {
    pub(crate) proxy_tx: Sender<CrusterWrapper>
}

#[async_trait]
impl HttpHandler for CrusterHandler {
    async fn handle_request(
        &mut self,
        _ctx: &HttpContext,
        req: Request<Body>
    ) -> RequestOrResponse
    {
        // println!("{:?}", &req);
        let (wrapper, new_req) = HyperRequestWrapper::from_hyper(req).await;
        // TODO: handle error in a better way
        // self.proxy_tx.send(CrusterWrapper::Request(wrapper));
        match self.proxy_tx.send(CrusterWrapper::Request(wrapper)).await {
            Ok(_) => RequestOrResponse::Request(new_req),
            Err(e) => panic!("Could not send: {}", e)
        }
    }

    async fn handle_response(&mut self, _ctx: &HttpContext, res: Response<Body>) -> Response<Body> {
        // println!("{:?}", res);
        res
    }
}
