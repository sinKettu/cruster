use http::HeaderMap;
use hudsucker::{
    hyper::{Body, Request, Response, self},
};

// use log::debug;

use crate::CrusterError;
use bstr::ByteSlice;
use std::fmt::Display;

#[derive(Clone, Debug)]
pub(crate) struct HyperRequestWrapper {
    pub(crate) uri: String,
    pub(crate) method: String,
    pub(crate) version: String,
    pub(crate) headers: hyper::HeaderMap,
    pub(crate) body: Vec<u8>
}

impl Display for HyperRequestWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut headers = String::default();
        for (k, v) in self.headers.iter() {
            headers = format!(
                "{}{}: {}\r\n",
                headers,
                k.as_str(),
                v.to_str().unwrap()
            );
        }
        
        write!(
            f,
            "{} {} {}\r\n{}\r\n{}",
            self.method.as_str(),
            self.get_request_path().as_str(),
            self.version.as_str(),
            headers,
            self.body.to_str_lossy()
        )
    }
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

impl Display for HyperResponseWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut headers = String::default();
        for (k, v) in self.headers.iter() {
            let header_line = format!("{}: {}\r\n", k.as_str(), v.to_str().unwrap());
            headers.push_str(&header_line);
        }

        write!(
            f,
            "{} {}\r\n{}\r\n{}",
            &self.version,
            &self.status,
            headers,
            self.body.to_str_lossy()
        )
    }
}

impl HyperResponseWrapper {
    pub(crate) async fn from_hyper(rsp: Response<Body>) -> Result<(Self, Response<Body>), CrusterError> {

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
                            body_compressed = BodyCompressedWith::NONE;
                        }
                    },
                    Err(_) => {
                        // nothing to do here
                        // add debug message
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

    // TODO: rewrite as trait implementation
    pub(crate) fn to_string(&self) -> String {
        let mut result = format!("{} {}\r\n", self.version, self.status);
        for (k, v) in self.headers.iter() {
            let header_line = format!("{}: {}\r\n", k.as_str(), v.to_str().unwrap());
            result.push_str(&header_line);
        }

        // TODO: rewrite with compression cases
        result.push_str("\r\n");
        result.push_str(self.body.to_str_lossy().as_ref());
        return result;
    }
}

// ---------------------------------------------------------------------------------------------- //

#[derive(Clone, Debug)]
pub(crate) enum CrusterWrapper {
    Request(HyperRequestWrapper),
    Response(HyperResponseWrapper)
}
