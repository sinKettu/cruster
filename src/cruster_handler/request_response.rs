use std::os::macos::raw::stat;
use http::HeaderMap;
use hudsucker::{
    hyper::{Body, Request, Response, self},
};
use log::debug;

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
        // TODO сделать через parts
        let (parts, body) = req.into_parts();
        let uri = parts.uri.clone().to_string();
        let method = parts.method.clone().to_string();
        let headers = parts.headers.clone();

        // TODO: Debug implemetned for version, use it
        let version = match parts.version {
            hyper::Version::HTTP_11 => "HTTP/1.1".to_string(),
            hyper::Version::HTTP_09 => "HTTP/0.1".to_string(),
            hyper::Version::HTTP_10 => "HTTP/1.0".to_string(),
            hyper::Version::HTTP_2 => "HTTP/2".to_string(),
            hyper::Version::HTTP_3 => "HTTP/2".to_string(),
            _ => "HTTP/UNKNOWN".to_string()
        };

        let body = match hyper::body::to_bytes(body).await {
            Ok(body_bytes) => body_bytes,
            Err(e) => {
                debug!("- CRUSTER - Could not read request body for '{} {}', reason: {:?}", &method, &uri, e);
                // This is not the best way to do it
                hyper::body::Bytes::new()
            }
        };


        let reconstructed_request = Request::from_parts(parts, Body::from(body.clone()));
        let request_wrapper = HyperRequestWrapper {
            uri,
            method,
            version,
            headers,
            body: body.to_vec()
        };

        return (request_wrapper, reconstructed_request);
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

        let version = match rsp_parts.version.clone() {
            hyper::Version::HTTP_11 => "HTTP/1.1".to_string(),
            hyper::Version::HTTP_09 => "HTTP/0.1".to_string(),
            hyper::Version::HTTP_10 => "HTTP/1.0".to_string(),
            hyper::Version::HTTP_2 => "HTTP/2".to_string(),
            hyper::Version::HTTP_3 => "HTTP/2".to_string(),
            _ => "HTTP/UNKNOWN".to_string() // TODO: Think once more
        };

        // Copy headers and determine if body is compressed
        let mut headers = HeaderMap::new();
        let mut body_compressed: BodyCompressedWith = BodyCompressedWith::NONE;
        for (k, v) in &rsp_parts.headers.clone() {
            headers.insert(k.clone(), v.clone());
            if k.as_str().to_lowercase() == "content-encoding" {
                match v.to_str() {
                    Ok(s) => {
                        if s.contains("gzip") {
                            body_compressed = BodyCompressedWith::GZIP;
                        } else if s.contains("deflate") {
                            body_compressed = BodyCompressedWith::DEFLATE;
                        }
                    },
                    Err(_e) => { () }
                }
            }
        }

        let body = match hyper::body::to_bytes(rsp_body).await {
            Ok(body_bytes) => body_bytes,
            Err(e) => {
                debug!("- CRUSTER - Could not read response body, reason: {:?}", e);
                // This is not the best way to do it
                hyper::body::Bytes::new()
            }
        };

        let reconstructed_body = Body::from(body.clone());
        let reconstructed_response = Response::from_parts(rsp_parts, reconstructed_body);
        let response_wrapper = HyperResponseWrapper {
            status,
            version,
            headers,
            body: body.to_vec(),
            body_compressed
        };

        return (response_wrapper, reconstructed_response);
    }
}

// ---------------------------------------------------------------------------------------------- //

pub(crate) enum CrusterWrapper {
    Request(HyperRequestWrapper),
    Response(HyperResponseWrapper)
}
