use std::fmt::format;
use http::HeaderMap;
use hudsucker::{
    hyper::{Body, Request, Response, self},
};
use log::debug;
use tokio::sync::mpsc::Sender;
use crate::CrusterError;


#[derive(Clone, Debug)]
pub(crate) struct HyperRequestWrapper {
    pub(crate) uri: String,
    pub(crate) method: String,
    pub(crate) version: String,
    pub(crate) headers: hyper::HeaderMap,
    pub(crate) body: Vec<u8>
}

impl HyperRequestWrapper {
    pub(crate) async fn from_hyper(req: Request<Body>) -> Result<(Self, Request<Body>), CrusterError> {
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
                return Err(CrusterError::from(e));
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

        return Ok((request_wrapper, reconstructed_request));
    }

    pub(crate) fn get_request_path(&self) -> String {
        let path_list = self.uri
            .split("/")
            .skip(3)
            .collect::<Vec<&str>>();
        format!("/{}", path_list.join("/"))
    }

    pub(crate) fn get_host(&self) -> String {
        match self.headers.get("Host") {
            Some(h) => {
                h
                    .clone()
                    .to_str()
                    .unwrap()
                    .to_string()
            },
            None => {
                self.uri
                    .split("/")
                    .skip(2)
                    .take(1)
                    .collect::<Vec<&str>>()[0]
                    .to_string()
            }
        }
    }
}

// -----------------------------------------------------------------------------------------------//

#[derive(Clone, Debug)]
pub(crate) enum BodyCompressedWith {
    GZIP,
    DEFLATE,
    BR,
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
    pub(crate) async fn from_hyper(
            rsp: Response<Body>,
            err_pipe: Option<& Sender<CrusterError>>) -> Result<(Self, Response<Body>), CrusterError> {

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
                        }
                        else if s.contains("deflate") {
                            body_compressed = BodyCompressedWith::DEFLATE;
                        }
                        else if s.contains("br") {
                            body_compressed = BodyCompressedWith::BR;
                        }
                        else {
                            if let Some(err_tx) = err_pipe {
                                let err_send_result = err_tx
                                    .send(CrusterError::UnknownResponseBodyEncoding(
                                        format!("Found unknown encoding for response body: {}", s))
                                    )
                                    .await;
                                if let Err(send_err) = err_send_result {
                                    panic!("Cannot send error about encoding to UI: {}", send_err.to_string());
                                }
                            }
                        }
                    },
                    Err(e) => {
                        if let Some(err_tx) = err_pipe {
                            let err_send_result = err_tx.send(e.into()).await;
                            if let Err(send_err) = err_send_result {
                                panic!("Cannot send error message to UI thread: {}", send_err.to_string());
                            }
                        }
                    }
                }
            }
        }

        let body = match hyper::body::to_bytes(rsp_body).await {
            Ok(body_bytes) => body_bytes,
            Err(e) => return Err(e.into())
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

        return Ok((response_wrapper, reconstructed_response));
    }

    pub(crate) fn get_length(&self) -> String {
        match self.headers.get("Content-Length") {
            Some(length) => {
                length
                    .to_str()
                    .unwrap()
                    .to_string()
            }
            None => {
                self.body.len().to_string()
            }
        }
    }
}

// ---------------------------------------------------------------------------------------------- //

pub(crate) enum CrusterWrapper {
    Request(HyperRequestWrapper),
    Response(HyperResponseWrapper)
}
