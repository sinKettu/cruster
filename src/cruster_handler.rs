use tokio::sync::mpsc::Sender;
use hudsucker::{
    async_trait::async_trait,
    hyper::{Body, Request, Response, self},
    HttpHandler,
    RequestOrResponse,
    HttpContext
};

#[derive(Clone, Debug)]
pub(crate) struct HyperRequestWrapper {
    pub(crate) uri: String,
    pub(crate) method: String,
    pub(crate) version: String,
    pub(crate) headers: hyper::HeaderMap,
    pub(crate) body: Vec<u8>
}

impl HyperRequestWrapper {
    pub(crate) async fn new(mut req: Request<Body>) -> (Self, Request<Body>) {
        let uri = req.uri().to_string();
        let method = req.method().to_string();
        let version = match req.version() {
            hyper::Version::HTTP_11 => "HTTP/1.1".to_string(),
            hyper::Version::HTTP_09 => "HTTP/0.1".to_string(),
            hyper::Version::HTTP_10 => "HTTP/1.0".to_string(),
            hyper::Version::HTTP_2 => "HTTP/2".to_string(),
            hyper::Version::HTTP_3 => "HTTP/2".to_string(),
            _ => "HTTP/UNKNOWN".to_string()
        };
        let headers = req.headers().to_owned();
        let body = hyper::body::to_bytes(req.body_mut())
            .await
            .unwrap()
            .to_vec();

        let mut new_request = hyper::Request::builder()
            .uri(hyper::Uri::try_from(uri.as_str()).unwrap())
            .method(hyper::Method::from_bytes(method.as_bytes()).unwrap())
            .version(hyper::Version::from(req.version()));
        for (k, v) in &headers { new_request = new_request.header(k, v); }
        let new_request = new_request.body(hyper::Body::from(body.clone())).unwrap();

        return (
            HyperRequestWrapper {
                uri,
                method,
                version,
                headers,
                body
            },
            new_request
        )
    }
}

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
        let (wrapper, new_req) = HyperRequestWrapper::new(req).await;
        // TODO: handle error in a better way
        self.proxy_tx.send(wrapper).await.unwrap();
        RequestOrResponse::Request(new_req)
    }

    async fn handle_response(&mut self, _ctx: &HttpContext, res: Response<Body>) -> Response<Body> {
        println!("{:?}", res);
        res
    }
}
