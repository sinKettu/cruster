use hudsucker::{
    async_trait::async_trait,
    hyper::{Body, Request, Response},
    HttpHandler,
    RequestOrResponse,
    HttpContext
};

#[derive(Clone)]
pub(crate) struct CrusterHandler;

#[async_trait]
impl HttpHandler for CrusterHandler {
    async fn handle_request(
        &mut self,
        _ctx: &HttpContext,
        req: Request<Body>
    ) -> RequestOrResponse
    {
        println!("{:?}", req);
        RequestOrResponse::Request(req)
    }

    async fn handle_response(&mut self, _ctx: &HttpContext, res: Response<Body>) -> Response<Body> {
        println!("{:?}", res);
        res
    }
}
