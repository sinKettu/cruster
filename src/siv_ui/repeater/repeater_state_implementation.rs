// use log::debug;
use base64;
use bstr::ByteSlice;
use cursive::views::TextContent;
use http::{HeaderMap, header::HeaderName, HeaderValue, Request};

use regex::Regex;
use std::str::FromStr;
use hyper::{body, Body};

use crate::utils::CrusterError;
use super::{RepeaterStateSerializable, RepeaterState};

impl From<&RepeaterState> for RepeaterStateSerializable {
    fn from(rs: &RepeaterState) -> Self {
        let rsp_content = rs.response.get_content();
        let rsp_raw = rsp_content.source();
        let rsp = if rsp_raw.is_empty() {
            None
        }
        else {
            Some(base64::encode(rsp_raw.as_bytes()))
        };

        RepeaterStateSerializable {
            name: rs.name.clone(),
            request: base64::encode(rs.request.as_bytes()),
            response: rsp,
            parameters: rs.parameters.clone()
        }
    }
}

impl TryFrom<RepeaterStateSerializable> for RepeaterState {
    type Error = CrusterError;
    fn try_from(rss: RepeaterStateSerializable) -> Result<Self, Self::Error> {
        let response = match rss.response.as_ref() {
            Some(rsp) => {
                let rsp_raw = base64::decode(rsp).unwrap();
                TextContent::new(rsp_raw.to_str().unwrap())
            },
            None => {
                TextContent::new("")
            }
        };
        
        let req_raw = match base64::decode(rss.request) {
            Ok(req) => {
                req
            },
            Err(e) => {
                return Err(e.into());
            }
        };

        let req_str = match String::from_utf8(req_raw) {
            Ok(req) => {
                req
            },
            Err(e) => {
                return Err(CrusterError::from(e));
            }
        };

        Ok(
            RepeaterState {
                name: rss.name,
                request: req_str,
                response,
                saved_headers: HeaderMap::default(),
                redirects_reached: 0,
                parameters: rss.parameters
            }
        )
    }
}

impl RepeaterState {
    // TODO: all regexes from this method can be reviewed
    // May be will be rewritten with special parser module
    pub(super) fn make_http_request(&self) -> Result<Request<Body>, CrusterError> {
        let req_fl = self.request.splitn(2, "\r\n").collect::<Vec<&str>>()[0];
        let fl_regex = Regex::new(r"^(?P<method>[\w]+) (?P<path>\S+) (?P<version>HTTP/(\d+\.)?\d+)$").unwrap();

        let (method, uri, version) =  match fl_regex.captures(req_fl) {
            Some(captures) => {
                let method = &captures["method"];
                let path = &captures["path"];
                let version = &captures["version"];

                let scheme = if self.parameters.https { "https" } else { "http" };
                let hostname = self.parameters.address.as_str();
                let uri = format!("{}://{}{}", scheme, hostname, path);

                (method.to_string(), uri, version.to_string())
            },
            None => {
                let err_str = format!("Could parse first line of request in repeater: {}", req_fl.clone());
                return Err(CrusterError::RegexError(err_str));
            }
        };

        let version = if version == "HTTP/0.9" {
            http::version::Version::HTTP_09
        }
        else if version == "HTTP/1.0" {
            http::version::Version::HTTP_10
        }
        else if version == "HTTP/1.1" {
            http::version::Version::HTTP_11
        }
        else if version == "HTTP/2" || version == "HTTP/2.0" {
            http::version::Version::HTTP_2
        }
        else if version == "HTTP/3" || version == "HTTP/3.0" {
            http::version::Version::HTTP_3
        }
        else {
            let err_str = format!("Unknown HTTP version of request in repeater: {}", &version);
            return Err(CrusterError::UndefinedError(err_str));
        };

        let mut request_builder = hyper::Request::builder()
            .method(method.as_str())
            .uri(uri)
            .version(version);

        let header_re = Regex::new(r"^(?P<name>[\d\w_\-]+): (?P<val>.*)$").unwrap();
        let mut body = String::with_capacity(self.request.len());
        let mut the_following_is_body = false;
        for line in self.request.split("\r\n").skip(1) {
            if line.is_empty() {
                the_following_is_body = true;
                continue;
            }

            if the_following_is_body {
                body.push_str(line);
                body.push_str("\r\n");
                continue;
            }

            match header_re.captures(line) {
                Some(cap) => {
                    let str_name = &cap["name"];
                    let name = match HeaderName::from_str(str_name) {
                        Ok(header_name) => {
                            header_name
                        },
                        Err(err) => {
                            return Err(err.into())
                        }
                    };

                    // TODO: parse something like \x0e in headers
                    let str_val = &cap["val"];
                    let val = match HeaderValue::from_str(str_val) {
                        Ok(header_value) => {
                            header_value
                        },
                        Err(err) => {
                            return Err(err.into())
                        }
                    };

                    match request_builder.headers_mut() {
                        Some(headers) => {
                            headers.insert(name, val);
                        },
                        None => {
                            let err = "Unknown error, while trying to parse request in repeater. Please check request's syntax".to_string();
                            return Err(CrusterError::UndefinedError(err));
                        }
                    }
                    
                },
                None => {
                    let err = CrusterError::RegexError(format!("Could not parse headers in repeater from {}", line));
                    return Err(err);
                }
            }
        }

        return match request_builder.body(body::Body::from(body)) {
            Ok(request) => {
                Ok(request)
            },
            Err(e) => {
                let err = CrusterError::HyperRequestBuildingError(format!("Could build request: {}", e.to_string()));
                Err(err)
            }
        };
    }

    // pub(super) fn make_request_to_redirect(&mut self, next_uri: &str) -> Result<Request<Body>, CrusterError> {
    //     let mut request_builder = hyper::Request::builder()
    //         .uri(next_uri);

    //     if let None = request_builder.headers_ref() {
    //         let err = "Unknown error, while trying to parse request in repeater. Please check request's syntax".to_string();
    //         return Err(CrusterError::UndefinedError(err));
    //     }

    //     for (k, v) in self.saved_headers.iter() {
    //         request_builder.headers_mut().unwrap().insert(k, v.clone());
    //     }

    //     let splited: Vec<&str> = next_uri.split("/").take(3).collect();
    //     let host = splited[2].to_string();

    //     request_builder
    //         .headers_mut()
    //         .unwrap()
    //         .insert("host", HeaderValue::from_str(&host).unwrap());

    //     let request = request_builder.body(Body::empty()).unwrap();
    //     let possible_wrapper = thread::spawn(move || {
    //         let runtime = Runtime::new().unwrap();
    //         let wrapper = runtime.block_on(HyperRequestWrapper::from_hyper(request));
    //         return wrapper;
    //     }).join().unwrap();

    //     return match possible_wrapper {
    //         Ok((wrapper, request)) => {
    //             self.request = wrapper.to_string();
    //             Ok(request)
    //         },
    //         Err(err) => {
    //             Err(err)
    //         }
    //     };
    // }
}
