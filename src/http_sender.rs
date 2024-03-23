use std::str::FromStr;

use http::{HeaderValue, HeaderMap};
use reqwest::{Request, Response, Client, Method, Url, Version};

use crate::{cruster_proxy::request_response::{HyperRequestWrapper, HyperResponseWrapper}, utils::CrusterError};


pub(crate) async fn send_request_from_wrapper(wrapper: &HyperRequestWrapper, max_redirects: usize) -> Result<(HyperResponseWrapper, bool), CrusterError> {
    let client = Client::new();
    let method = Method::from_str(&wrapper.method)?;
    let url = match Url::from_str(&wrapper.uri) {
        Ok(url) => {
            url
        },
        Err(err) => {
            return Err(CrusterError::CouldParseRequestPathError(err.to_string()));
        }
    };

    let version = &wrapper.version;
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
        // Crutch!
        Version::HTTP_11
        // return Err(
        //     CrusterError::HTTPBuildingError("Repeater does not support HTTP/2 neither HTTP/3, please set HTTP/1.1 version for request manually".to_string())
        // );
    }
    else if version == "HTTP/3" || version == "HTTP/3.0" {
        // Crutch!
        Version::HTTP_11
        // return Err(
        //     CrusterError::HTTPBuildingError("Repeater does not support HTTP/2 neither HTTP/3, please set HTTP/1.1 version for request manually".to_string())
        // );
    }
    else {
        let err_str = format!("Unknown HTTP version of request in repeater: {}", &version);
        return Err(CrusterError::UndefinedError(err_str));
    };

    let headers = wrapper.headers.clone();
    let body = wrapper.body.clone();

    let request = client
        .request(method, url)
        .version(version)
        .headers(headers)
        .body(body)
        .build()?;

    let (response, rc_exceeded) = send_reqwest(request, max_redirects).await?;
    return Ok(
        (
            HyperResponseWrapper::from_reqwest(response).await?,
            rc_exceeded
        )
    );
}


async fn follow_redirect(response: Response, client: &Client, mut redirects_counter: usize, cookie: Option<&HeaderValue>) -> Result<(Response, bool), CrusterError> {
    let mut current_response = response;

    while redirects_counter > 0 && current_response.status().is_redirection() {
        let location_str = current_response.headers().get("location");
        if let None = location_str {
            return Err(
                CrusterError::HeaderValueParseError("Found redirection, but could not find 'Location' header in response".to_string())
            );
        }

        let location_str = location_str.unwrap().to_str()?;
        let location_url = match reqwest::Url::parse(location_str) {
            Ok(url) => {
                url
            },
            Err(err) => {
                return Err(CrusterError::HeaderValueParseError(format!("Could not parse location URL for redirect from '{}'", location_str)));
            }
        };

        let mut headers = HeaderMap::new();
        headers.insert("referer", HeaderValue::from_str(current_response.url().as_str())?);
        if let Some(cookie) = cookie {
            headers.insert("cookie", cookie.clone());
        }

        current_response = client.request(reqwest::Method::GET, location_url)
            .headers(headers)
            .send()
            .await?;

        redirects_counter -= 1;
    }

    if redirects_counter == 0 && current_response.status().is_redirection() {
        // True means 'The statement "Redirects counter exceeded" is TRUE'
        Ok((current_response, true))
    }
    else {
        // False means 'The statement "Redirects counter exceeded" is FALSE'
        Ok((current_response, false))
    }
}


pub(crate) async fn send_reqwest(req: Request, max_redirects: usize) -> Result<(Response, bool), CrusterError> {
    let cookie = if let Some(cookie) = req.headers().get("cookie") {
        Some(cookie.to_owned())
    }
    else {
        None
    };

    let client = Client::builder()
        .use_rustls_tls()
        .redirect(reqwest::redirect::Policy::none())
        .http1_only()
        .build()?;

    let response = client.execute(req).await?;

    if response.status().is_redirection() && max_redirects > 0 {
        let counter = max_redirects.saturating_sub(1);
        let (response, rc_exceeded) = follow_redirect(response, &client, counter, cookie.as_ref()).await?;
        return Ok((response, rc_exceeded));
    }

    return Ok((response, false));
}