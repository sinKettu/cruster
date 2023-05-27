use hudsucker::{
    hyper::{
        Body,
        Request,
        Response,
        self
    },
    decode_response
};

// use log::debug;

use http::HeaderMap;
use bstr::ByteSlice;
use std::{fmt::Display, borrow::Cow};
use std::ffi::CString;

use crate::CrusterError;
use regex::Regex;

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
        let mut keys_list: Vec<&str> = self.headers
            .keys()
            .into_iter()
            .map(|k| {
                k.as_str()
            })
            .collect();

        keys_list.sort();
        for key in keys_list {
            let v_iter = self.headers
                .get_all(key)
                .iter()
                .map(|val| {
                    val.as_bytes().to_str_lossy()
                })
                .collect::<Vec<Cow<str>>>()
                .join("; ");

            headers = format!(
                "{}{}: {}\r\n",
                headers,
                key,
                v_iter
            );
        }

        // Crutch because of binary string which are incompatible with c-strings in cursive
        // TODO: check with crossterm, may be should use feature-toggle here
        let body = self.body.to_str_lossy().to_string();
        let body = if CString::new(body.as_bytes()).is_ok() {
            body
        }
        else {
            "--- INCOMPATIBLE SET OF BYTES ---".to_string()
        };
        
        write!(
            f,
            "{} {} {}\r\n{}\r\n{}",
            self.method.as_str(),
            self.get_request_path().as_str(),
            self.version.as_str(),
            headers,
            body
        )
    }
}

impl HyperRequestWrapper {
    pub(crate) async fn from_hyper(req: Request<Body>) -> Result<(Self, Request<Body>), CrusterError> {
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

    pub(crate) async fn from_reqwest(req: reqwest::Request) -> Result<Self, CrusterError> {
        let uri = req.url().to_string();
        let method = req.method().to_string();
        let headers = req.headers().clone();

        let version = match req.version() {
            reqwest::Version::HTTP_11 => "HTTP/1.1".to_string(),
            reqwest::Version::HTTP_09 => "HTTP/0.1".to_string(),
            reqwest::Version::HTTP_10 => "HTTP/1.0".to_string(),
            reqwest::Version::HTTP_2 => "HTTP/2".to_string(),
            reqwest::Version::HTTP_3 => "HTTP/2".to_string(),
            _ => "HTTP/UNKNOWN".to_string()
        };

        let body: Vec<u8> = match req.body() {
            Some(body) => {
                body.as_bytes().unwrap().to_vec()
            },
            None => {
                vec![]
            }
        };

        let wrapper = HyperRequestWrapper {
            uri,
            method,
            version,
            headers,
            body
        };

        Ok(wrapper)
    }

    pub(crate) fn get_request_path(&self) -> String {
        let path_list = self.uri
            .split("/")
            .skip(3)
            .collect::<Vec<&str>>();
        format!("/{}", path_list.join("/"))
    }

    pub(crate) fn get_request_path_without_query(&self) -> Result<String, CrusterError> {
        let after_split = self.uri.splitn(4, "/").collect::<Vec<&str>>();
        let path_with_query_option = after_split.get(3);

        return match path_with_query_option {
            Some(path_with_query) => {
                let after_second_split = path_with_query.splitn(2, "?").collect::<Vec<&str>>();
                let path_without_query_option = after_second_split.get(0);

                match path_without_query_option {
                    Some(path_without_query) => {
                        Ok(format!("/{}", path_without_query))
                    },
                    None => {
                        Err(CrusterError::CouldParseRequestPathError(format!("Could not parse path at {}", self.uri)))
                    }
                }
            },
            None => {
                Err(CrusterError::CouldParseRequestPathError(format!("Could not parse path at {}", self.uri)))
            }
        };
    }

    pub(crate) fn get_query(&self) -> Option<String> {
        let after_split: Vec<&str> = self.uri.splitn(2, "?").collect();

        return match after_split.get(1) {
            Some(query) => {
                Some(format!("?{}", query))
            },
            None => {
                None
            }
        };
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

    pub(crate) fn get_hostname(&self) -> String {
        if self.method == "CONNECT" {
            return self.uri.clone();
        }
        else {
            return self.uri
                .split("/")
                .skip(2)
                .take(1)
                .collect::<Vec<&str>>()[0]
                .to_string()
        }
    }

    pub(crate) fn get_scheme(&self) -> String {
        let split_result: Vec<&str> = self.uri.splitn(2, ":").collect();
        return match split_result.get(0) {
            Some(scheme) => {
                format!("{}://", scheme)
            },
            None => {
                "://".to_string()
            }
        };
    }

    pub(crate) fn serach_with_re(&self, re: &Regex) -> bool {
        let fl = format!("{} {} {}\r\n", &self.method, &self.uri, &self.version);
        if re.find(&fl).is_some() {
            return true;
        }

        let found_in_headers: bool = self.headers
            .iter()
            .any(|(k, v)| {
                let hl = format!("{}: {}", k.as_str(), v.as_bytes().to_str_lossy().as_ref());
                re.find(&hl).is_some()
            });
        
        if found_in_headers {
            return true;
        }

        let body = self.body.as_slice().to_str_lossy();
        return re.find(&body).is_some();
    }
}

// -----------------------------------------------------------------------------------------------//

#[derive(Clone, Debug)]
pub(crate) struct HyperResponseWrapper {
    pub(crate) status: String,
    pub(crate) version: String,
    pub(crate) headers: hyper::HeaderMap,
    pub(crate) body: Vec<u8>,
    // pub(crate) body_compressed: BodyCompressedWith
}

impl Display for HyperResponseWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut headers = String::default();
        let mut keys_list: Vec<&str> = self.headers
            .keys()
            .into_iter()
            .map(|k| {
                k.as_str()
            })
            .collect();

        keys_list.sort();
        for key in keys_list {
            let v_iter = self.headers
                .get_all(key)
                .iter()
                .map(|val| {
                    val.as_bytes().to_str_lossy()
                })
                .collect::<Vec<Cow<str>>>()
                .join("; ");

            let header_line = format!("{}: {}\r\n", key, v_iter);
            headers.push_str(&header_line);
        }

        // Crutch because of binary string which are incompatible with c-strings in cursive
        // TODO: check with crossterm, may be should use feature-toggle here
        let body = self.body.to_str_lossy().to_string();
        let body = if CString::new(body.as_bytes()).is_ok() {
            body
        }
        else {
            "--- INCOMPATIBLE SET OF BYTES ---".to_string()
        };

        write!(
            f,
            "{} {}\r\n{}\r\n{}",
            &self.version,
            &self.status,
            headers,
            body
        )
        // write!(f, "{}", "RESPONSE")
    }
}

impl HyperResponseWrapper {
    pub(crate) async fn from_hyper(rsp: Response<Body>) -> Result<(Self, Response<Body>), CrusterError> {
        let rsp = decode_response(rsp);

        if let Err(err) = rsp {
            return Err(CrusterError::HudSuckerError(err.to_string()));
        }

        let (rsp_parts, rsp_body) = rsp.unwrap().into_parts();
        let status = rsp_parts.status.clone().to_string();

        let version = match rsp_parts.version.clone() {
            hyper::Version::HTTP_11 => "HTTP/1.1".to_string(),
            hyper::Version::HTTP_09 => "HTTP/0.1".to_string(),
            hyper::Version::HTTP_10 => "HTTP/1.0".to_string(),
            hyper::Version::HTTP_2 => "HTTP/2".to_string(),
            hyper::Version::HTTP_3 => "HTTP/2".to_string(),
            _ => "HTTP/UNKNOWN".to_string() // TODO: Think once more
        };

        // Copy headers
        let mut headers = HeaderMap::new();
        for (k, v) in &rsp_parts.headers.clone() {
            headers.insert(k.clone(), v.clone());
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
            // body_compressed
        };

        return Ok((response_wrapper, reconstructed_response));
    }

    pub(crate) async fn from_reqwest(rsp: reqwest::Response) -> Result<Self, CrusterError> {
        let status = rsp.status().to_string();
        let wrapper_headers = rsp.headers().clone();

        let version = match rsp.version() {
            reqwest::Version::HTTP_11 => "HTTP/1.1".to_string(),
            reqwest::Version::HTTP_09 => "HTTP/0.1".to_string(),
            reqwest::Version::HTTP_10 => "HTTP/1.0".to_string(),
            reqwest::Version::HTTP_2 => "HTTP/2".to_string(),
            reqwest::Version::HTTP_3 => "HTTP/2".to_string(),
            _ => "HTTP/UNKNOWN".to_string() // TODO: Think once more
        };

        let body = rsp.bytes().await?.to_vec();

        let wrapper = HyperResponseWrapper {
            status,
            version,
            headers: wrapper_headers,
            body
        };

        return Ok(wrapper);
    }

    pub(crate) fn get_length(&self) -> usize {
        match self.headers.get("Content-Length") {
            Some(length) => {
                length
                    .to_str()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap()
            }
            None => {
                self.body.len()
            }
        }
    }

    pub(crate) fn serach_with_re(&self, re: &Regex) -> bool {
        let fl = format!("{} {}\r\n", &self.version, &self.status);
        if re.find(&fl).is_some() {
            return true;
        }

        let found_in_headers: bool = self.headers
            .iter()
            .any(|(k, v)| {
                let hl = format!("{}: {}", k.as_str(), v.as_bytes().to_str_lossy().as_ref());
                re.find(&hl).is_some()
            });
        
        if found_in_headers {
            return true;
        }

        let body = self.body.as_slice().to_str_lossy();
        return re.find(&body).is_some();
    }
}
