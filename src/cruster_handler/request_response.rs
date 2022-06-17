use http::HeaderMap;
use hudsucker::{
    hyper::{Body, Request, Response, self},
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
    pub(crate) async fn from_hyper(mut req: Request<Body>) -> (Self, Request<Body>) {
        let uri = req.uri().to_string();
        let method = req.method().to_string();
        let version = match req.version() {
            hyper::Version::HTTP_11 => "HTTP/1.1".to_string(),
            hyper::Version::HTTP_09 => "HTTP/0.1".to_string(),
            hyper::Version::HTTP_10 => "HTTP/1.0".to_string(),
            hyper::Version::HTTP_2 => "HTTP/2".to_string(),
            hyper::Version::HTTP_3 => "HTTP/2".to_string(),
            _ => "HTTP/UNKNOWN".to_string() // TODO: Think once more
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

// -----------------------------------------------------------------------------------------------//

#[derive(Clone, Debug)]
pub(crate) enum BodyCompressedWith {
    GZIP,
    DEFLATE,
    NONE
}

#[derive(Clone, Debug)]
pub(crate) struct HyperResponseWrapper {
    pub(crate) status: String,
    pub(crate) version: String,
    pub(crate) headers: hyper::HeaderMap,
    pub(crate) body: Vec<u8>,
    pub(crate) body_compressed: BodyCompressedWith
}

impl HyperResponseWrapper {
    pub(crate) async fn from_hyper(rsp: Response<Body>) -> (Self, Response<Body>) {
        let (rsp_parts, rsp_body) = rsp.into_parts();
        let status = rsp_parts.status.clone().to_string();
        let version = match rsp_parts.version {
            hyper::Version::HTTP_11 => "HTTP/1.1".to_string(),
            hyper::Version::HTTP_09 => "HTTP/0.1".to_string(),
            hyper::Version::HTTP_10 => "HTTP/1.0".to_string(),
            hyper::Version::HTTP_2 => "HTTP/2".to_string(),
            hyper::Version::HTTP_3 => "HTTP/2".to_string(),
            _ => "HTTP/UNKNOWN".to_string() // TODO: Think once more
        };

        let mut headers = HeaderMap::new();
        // Copy headers and determine if body is compressed
        let mut body_compressed: BodyCompressedWith = BodyCompressedWith::NONE;
        for (k, v) in &rsp_parts.headers {
            if k.as_str().to_lowercase() == "content-encoding" {
                match v.to_str() {
                    Ok(s) => {
                        if s.contains("gzip") {
                            body_compressed = BodyCompressedWith::GZIP;
                        } else if s.contains("deflate") {
                            body_compressed = BodyCompressedWith::DEFLATE;
                        }
                    },
                    Err(_e) => { todo!() }
                }
            }

            headers.insert(k.clone(), v.clone());
        }
        let body = hyper::body::to_bytes(rsp_body).await.unwrap().to_vec();
        let new_body = Body::from(body.clone());

        (
            HyperResponseWrapper {
                status,
                version,
                headers,
                body,
                body_compressed
            },
            Response::from_parts(rsp_parts, new_body)
        )
    }
}

// ---------------------------------------------------------------------------------------------- //

pub(crate) enum CrusterWrapper {
    Request(HyperRequestWrapper),
    Response(HyperResponseWrapper)
}
