pub(crate) mod request_response;

use request_response::HyperRequestWrapper;
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
    pub(crate) proxy_tx: Sender<HyperRequestWrapper>
}

/// TODO: find a way to copy hyper::Request
// impl CrusterHandler {
//     async fn clone_request(req: &Request<Body>) -> Request<Body> {
//         let (parts, body) = req.into_parts();
//         let cloned_version = parts.version.clone();
//         let cloned_method = parts.method.clone();
//         let cloned_uri = parts.uri.clone();
//         let hd = parts.headers.clone();
//         let new_body = hyper_body::to_bytes(body).await.unwrap();
//         let new_body = Body::from(new_body.clone());
//
//
//         let mut new_req = Request::builder()
//             .method(cloned_method)
//             .uri(cloned_uri)
//             .version(cloned_version);
//
//         for (k, v) in &hd {
//             new_req = new_req.header(k, v);
//         }
//
//         new_req.body(new_body).unwrap()
//     }
// }

#[async_trait]
impl HttpHandler for CrusterHandler {
    async fn handle_request(
        &mut self,
        _ctx: &HttpContext,
        req: Request<Body>
    ) -> RequestOrResponse
    {
        println!("{:?}", &req);
        // let (wrapper, new_req) = HyperRequestWrapper::from_hyper(req).await;
        // // TODO: handle error in a better way
        // self.proxy_tx.send(wrapper).await.unwrap();
        // RequestOrResponse::Request(new_req)
        RequestOrResponse::Request(req)
    }

    async fn handle_response(&mut self, _ctx: &HttpContext, res: Response<Body>) -> Response<Body> {
        println!("{:?}", res);
        res
    }
}
