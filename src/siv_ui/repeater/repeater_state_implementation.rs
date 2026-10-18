// use log::debug;
use base64;
use bstr::ByteSlice;
use cursive::views::TextContent;
use http::{HeaderMap, header::HeaderName, HeaderValue};

use regex::Regex;
use std::str::FromStr;
use hyper::Version;
use reqwest;

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
    pub(crate) fn make_reqwest(&self) -> Result<reqwest::Request, CrusterError> {
        let req_fl = self.request.splitn(2, "\n").collect::<Vec<&str>>()[0];
        let fl_regex = Regex::new(r"^(?P<method>\S+) (?P<path>\S+) (?P<version>HTTP/(\d+\.)?\d+)\s?$").unwrap();

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
                let err_str = format!("Could not parse first line of request in repeater: {}", req_fl);
                return Err(CrusterError::RegexError(err_str));
            }
        };

        let version = if version == "HTTP/0.9" {
            Version::HTTP_09
        }
        else if version == "HTTP/1.0" {
            Version::HTTP_10
        }
        else if version == "HTTP/1.1" {
            Version::HTTP_11
        }
        else if version == "HTTP/2" || version == "HTTP/2.0" {
            Version::HTTP_2
        }
        else if version == "HTTP/3" || version == "HTTP/3.0" {
            return Err(
                CrusterError::HTTPBuildingError(
                    "HTTP/3 requests are not supported by the stable client; use HTTP/2 or lower".to_string()
                )
            );
        }
        else {
            let err_str = format!("Unknown HTTP version of request in repeater: {}", &version);
            return Err(CrusterError::UndefinedError(err_str));
        };

        let method = reqwest::Method::from_str(&method)?;
        let url = match reqwest::Url::from_str(&uri) {
            Ok(url) => url,
            Err(err) => return Err(CrusterError::CouldParseRequestPathError(err.to_string()))
        };

        let client = reqwest::Client::new();
        let request = client.request(method, url)
            .version(version);

        let mut new_headers = HeaderMap::new();
        // Split only once: blank lines and trailing whitespace inside an edited
        // body must survive replay exactly as entered (including MCP edits).
        let (head, body) = self.request.split_once("\r\n\r\n")
            .filter(|(head, _)| !head.contains("\n\n"))
            .or_else(|| self.request.split_once("\n\n"))
            .unwrap_or((self.request.as_str(), ""));
        for line in head.split("\n").skip(1) {
            match line.trim_end_matches('\r').split_once(':') {
                Some((str_name, str_val)) => {
                    let name = match HeaderName::from_str(str_name) {
                        Ok(header_name) => {
                            header_name
                        },
                        Err(err) => {
                            return Err(err.into())
                        }
                    };

                    // TODO: parse something like \x0e in headers
                    let str_val = str_val.trim_matches([' ', '\t']);
                    let val = match HeaderValue::from_str(str_val) {
                        Ok(header_value) => {
                            header_value
                        },
                        Err(err) => {
                            return Err(err.into())
                        }
                    };

                    new_headers.append(name, val);
                },
                None => {
                    let err = CrusterError::RegexError(format!("Could not parse headers in repeater from {}", line));
                    return Err(err);
                }
            }
        }

        // reqwest calculates Content-Length from the final edited body. Keeping
        // the intercepted value is unsafe and can cause HTTP/2 stream resets.
        new_headers.remove("content-length");

        let request = request
            .headers(new_headers)
            .body(body.to_owned())
            .build()?;
        
        Ok(request)
    }
}
