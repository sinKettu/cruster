use std::{net::{IpAddr, SocketAddr}, str::FromStr};

use http::{HeaderValue, HeaderMap};
use reqwest::{Request, Response, Client, Method, Url, Version};

use crate::{cruster_proxy::request_response::{HyperRequestWrapper, HyperResponseWrapper}, utils::CrusterError};

fn headers_for_version(wrapper: &HyperRequestWrapper, version: Version) -> HeaderMap {
    let mut headers = wrapper.headers.clone();

    if version == Version::HTTP_2 {
        // HTTP/2 carries connection state in frames. Forwarding HTTP/1-only
        // connection headers can make strict servers reset the stream with
        // PROTOCOL_ERROR. Keep Host: hyper maps it to the HTTP/2 :authority
        // pseudo-header, which is required for virtual-host security testing.
        for forbidden in [
            "connection",
            "keep-alive",
            "proxy-connection",
            "transfer-encoding",
            "upgrade",
            // Its value has already been moved into the URL authority by
            // http2_client_and_url(); do not also encode it as a regular field.
            "host",
        ] {
            headers.remove(forbidden);
        }

        if headers.get("te").map(|value| value.as_bytes()) != Some(b"trailers") {
            headers.remove("te");
        }
    }

    // Let reqwest/hyper encode the body length. This prevents a stale length
    // copied from intercepted traffic from disagreeing with the edited body.
    headers.remove("content-length");
    headers
}

async fn http2_client_and_url(
    wrapper: &HyperRequestWrapper,
    mut url: Url,
) -> Result<(Client, Url), CrusterError> {
    let host_header = match wrapper.headers.get("host") {
        Some(value) => value.to_str()?.to_string(),
        None => return Ok((Client::builder().use_rustls_tls().redirect(reqwest::redirect::Policy::none()).build()?, url)),
    };
    let authority_url = Url::parse(&format!("{}://{}", url.scheme(), host_header))
        .map_err(|error| CrusterError::CouldParseRequestPathError(error.to_string()))?;
    let authority_host = authority_url.host_str()
        .ok_or_else(|| CrusterError::CouldParseRequestPathError(format!("Invalid Host header: {}", host_header)))?
        .to_string();
    let authority_port = authority_url.port();

    let connect_host = url.host_str()
        .ok_or_else(|| CrusterError::CouldParseRequestPathError("Request URL has no host".to_string()))?
        .to_string();
    let connect_port = url.port_or_known_default()
        .ok_or_else(|| CrusterError::CouldParseRequestPathError("Request URL has no known port".to_string()))?;
    let connect_addr = if let Ok(ip) = connect_host.parse::<IpAddr>() {
        SocketAddr::new(ip, connect_port)
    } else {
        tokio::net::lookup_host((connect_host.as_str(), connect_port))
            .await?
            .next()
            .ok_or_else(|| CrusterError::UndefinedError(format!("Could not resolve connection target {}", connect_host)))?
    };

    // For HTTP/2, Host is represented by :authority, not a regular header.
    // Put the desired virtual host into the URL, while resolve() keeps the TCP
    // connection pinned to the user-selected repeater address.
    url.set_host(Some(&authority_host))
        .map_err(|_| CrusterError::CouldParseRequestPathError(format!("Invalid Host header: {}", host_header)))?;
    url.set_port(authority_port)
        .map_err(|_| CrusterError::CouldParseRequestPathError(format!("Invalid Host port: {}", host_header)))?;

    let client = Client::builder()
        .use_rustls_tls()
        .redirect(reqwest::redirect::Policy::none())
        .resolve(&authority_host, connect_addr)
        .build()?;
    Ok((client, url))
}

pub(crate) async fn send_request_from_wrapper(wrapper: &HyperRequestWrapper, max_redirects: usize) -> Result<(HyperResponseWrapper, bool), CrusterError> {
    if wrapper.version == "HTTP/3" || wrapper.version == "HTTP/3.0" {
        return Err(CrusterError::HTTPBuildingError(
            "HTTP/3 requests are not supported by the stable client; use HTTP/2 or lower".to_string()
        ));
    }

    let method = Method::from_str(&wrapper.method)?;
    let mut url = match Url::from_str(&wrapper.uri) {
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
        Version::HTTP_2
    }
    else {
        let err_str = format!("Unknown HTTP version of request in repeater: {}", &version);
        return Err(CrusterError::UndefinedError(err_str));
    };

    let (client, rewritten_url) = if version == Version::HTTP_2 {
        http2_client_and_url(wrapper, url).await?
    } else {
        (
            Client::builder()
                .use_rustls_tls()
                .redirect(reqwest::redirect::Policy::none())
                .build()?,
            url,
        )
    };
    url = rewritten_url;

    let headers = headers_for_version(wrapper, version);
    let body = wrapper.body.clone();

    let request = client
        .request(method, url)
        .version(version)
        .headers(headers)
        .body(body)
        .build()?;

    let (response, rc_exceeded) = send_reqwest_with_client(request, max_redirects, &client).await?;
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
                return Err(CrusterError::HeaderValueParseError(format!("Could not parse location URL for redirect from '{}': {}", location_str, err)));
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
    let client = Client::builder()
        .use_rustls_tls()
        .redirect(reqwest::redirect::Policy::none())
        .build()?;
    send_reqwest_with_client(req, max_redirects, &client).await
}

async fn send_reqwest_with_client(req: Request, max_redirects: usize, client: &Client) -> Result<(Response, bool), CrusterError> {
    let cookie = if let Some(cookie) = req.headers().get("cookie") {
        Some(cookie.to_owned())
    }
    else {
        None
    };

    let response = client.execute(req).await?;

    if response.status().is_redirection() && max_redirects > 0 {
        let counter = max_redirects.saturating_sub(1);
        let (response, rc_exceeded) = follow_redirect(response, client, counter, cookie.as_ref()).await?;
        return Ok((response, rc_exceeded));
    }

    return Ok((response, false));
}
