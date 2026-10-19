// use log::debug;
use base64;
use cursive::views::TextContent;
use http::HeaderMap;

use hyper::Version;
use reqwest;
use std::str::FromStr;

use super::{RepeaterState, RepeaterStateSerializable};
use crate::utils::CrusterError;

impl From<&RepeaterState> for RepeaterStateSerializable {
    fn from(rs: &RepeaterState) -> Self {
        let rsp_content = rs.response.get_content();
        let rsp_raw = rsp_content.source();
        let rsp = if rsp_raw.is_empty() {
            None
        } else {
            Some(base64::encode(rsp_raw.as_bytes()))
        };

        RepeaterStateSerializable {
            name: rs.name.clone(),
            request: base64::encode(rs.request.as_bytes()),
            response: rsp,
            parameters: rs.parameters.clone(),
        }
    }
}

impl TryFrom<RepeaterStateSerializable> for RepeaterState {
    type Error = CrusterError;
    fn try_from(rss: RepeaterStateSerializable) -> Result<Self, Self::Error> {
        let response = match rss.response.as_ref() {
            Some(rsp) => {
                let rsp_raw = base64::decode(rsp)?;
                TextContent::new(String::from_utf8(rsp_raw)?)
            }
            None => TextContent::new(""),
        };

        let req_raw = match base64::decode(rss.request) {
            Ok(req) => req,
            Err(e) => {
                return Err(e.into());
            }
        };

        let req_str = match String::from_utf8(req_raw) {
            Ok(req) => req,
            Err(e) => {
                return Err(CrusterError::from(e));
            }
        };

        Ok(RepeaterState {
            name: rss.name,
            request: req_str,
            response,
            saved_headers: HeaderMap::default(),
            redirects_reached: 0,
            parameters: rss.parameters,
        })
    }
}

impl RepeaterState {
    pub(crate) fn make_reqwest(&self) -> Result<reqwest::Request, CrusterError> {
        let target = format!(
            "{}://{}/",
            if self.parameters.https {
                "https"
            } else {
                "http"
            },
            self.parameters.address
        );
        // Legacy CLI editors may omit the final blank line for an empty body.
        let raw = if crate::cli::repeater::parsing::split_raw(&self.request).is_none() {
            format!("{}\n\n", self.request.trim_end_matches(['\r', '\n']))
        } else {
            self.request.clone()
        };
        let wrapper = crate::cli::repeater::parsing::parse_raw(&raw, &target)?;
        let version = match wrapper.version.as_str() {
            "HTTP/0.9" => Version::HTTP_09,
            "HTTP/1.0" => Version::HTTP_10,
            "HTTP/1.1" => Version::HTTP_11,
            _ => Version::HTTP_2,
        };
        let mut headers = wrapper.headers;
        headers.remove("content-length");
        Ok(reqwest::Client::new()
            .request(reqwest::Method::from_str(&wrapper.method)?, &wrapper.uri)
            .version(version)
            .headers(headers)
            .body(wrapper.body)
            .build()?)
    }
}
