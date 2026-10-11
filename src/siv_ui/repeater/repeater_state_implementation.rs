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
        let fl_regex = Regex::new(r"^(?P<method>[\w]+) (?P<path>\S+) (?P<version>HTTP/(\d+\.)?\d+)\s?$").unwrap();

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
            return Err(
                CrusterError::HTTPBuildingError("Repeater does not support HTTP/2 neither HTTP/3, please set HTTP/1.1 version for request manually".to_string())
            );
        }
        else if version == "HTTP/3" || version == "HTTP/3.0" {
            return Err(
                CrusterError::HTTPBuildingError("Repeater does not support HTTP/2 neither HTTP/3, please set HTTP/1.1 version for request manually".to_string())
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
        let header_re = Regex::new(r"^(?P<name>[\d\w_\-]+): (?P<val>.*)$").unwrap();
        let mut body = String::with_capacity(self.request.len());
        let mut the_following_is_body = false;
        let mut first_body_line = true;
        for line in self.request.split("\n").skip(1) {
            let trimmed_line = line.trim_end();
            if trimmed_line.is_empty() {
                the_following_is_body = true;
                continue;
            }

            if the_following_is_body {
                body.push_str(line);

                if ! first_body_line {
                    body.push_str("\r\n");
                }
                else {
                    first_body_line = false;
                }
                
                continue;
            }

            match header_re.captures(trimmed_line) {
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

                    new_headers.insert(name, val);
                },
                None => {
                    let err = CrusterError::RegexError(format!("Could not parse headers in repeater from {}", line));
                    return Err(err);
                }
            }
        }

        let str_body_length = body.len().to_string();
        let _ = new_headers.insert("Content-Length", HeaderValue::from_str(&str_body_length)?);

        let request = request
            .headers(new_headers)
            .body(body)
            .build()?;
        
        Ok(request)
    }
}
