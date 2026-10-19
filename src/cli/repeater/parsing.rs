use crate::{cruster_proxy::request_response::HyperRequestWrapper, utils::CrusterError};
use http::{header::HeaderName, HeaderMap, HeaderValue};

fn invalid(error: impl ToString) -> CrusterError {
    CrusterError::HTTPBuildingError(error.to_string())
}

pub(crate) fn validate(request: &HyperRequestWrapper) -> Result<(), CrusterError> {
    request.method.parse::<http::Method>().map_err(invalid)?;
    let url = reqwest::Url::parse(&request.uri).map_err(invalid)?;
    if !matches!(url.scheme(), "http" | "https")
        || url.host_str().is_none()
        || url.fragment().is_some()
        || !url.username().is_empty()
        || url.password().is_some()
    {
        return Err(invalid(
            "Use an absolute HTTP(S) URL without credentials or fragment",
        ));
    }
    if !matches!(
        request.version.as_str(),
        "HTTP/0.9" | "HTTP/1.0" | "HTTP/1.1" | "HTTP/2" | "HTTP/2.0"
    ) {
        return Err(invalid(
            "Supported HTTP versions: HTTP/0.9, HTTP/1.0, HTTP/1.1, HTTP/2",
        ));
    }
    Ok(())
}

// Split only the header section: do not normalize or trim body whitespace.
pub(crate) fn parse_raw(raw: &str, target: &str) -> Result<HyperRequestWrapper, CrusterError> {
    let (head, body) = split_raw(raw)
        .ok_or_else(|| invalid("Raw request must contain a blank line between headers and body"))?;
    let mut lines = head.lines();
    let parts: Vec<_> = lines
        .next()
        .unwrap_or_default()
        .split_whitespace()
        .collect();
    if parts.len() != 3 || !parts[1].starts_with('/') {
        return Err(invalid(
            "Expected request line: METHOD /path?query HTTP/1.1 (origin-form target)",
        ));
    }
    let mut url = reqwest::Url::parse(target).map_err(invalid)?;
    let (path, query) = parts[1]
        .split_once('?')
        .map_or((parts[1], None), |(p, q)| (p, Some(q)));
    url.set_path(path);
    url.set_query(query);
    let mut headers = HeaderMap::new();
    for line in lines {
        let (name, value) = line
            .split_once(':')
            .ok_or_else(|| invalid(format!("Invalid header: {}", line)))?;
        headers.append(
            name.parse::<HeaderName>().map_err(invalid)?,
            HeaderValue::from_str(value.trim_matches([' ', '\t'])).map_err(invalid)?,
        );
    }
    let request = HyperRequestWrapper {
        uri: url.to_string(),
        method: parts[0].into(),
        version: parts[2].into(),
        headers,
        body: body.as_bytes().to_vec(),
    };
    validate(&request)?;
    Ok(request)
}

pub(crate) fn split_raw(raw: &str) -> Option<(&str, &str)> {
    raw.split_once("\r\n\r\n")
        .filter(|(head, _)| !head.contains("\n\n"))
        .or_else(|| raw.split_once("\n\n"))
}

pub(crate) fn render_raw(request: &HyperRequestWrapper) -> Result<String, CrusterError> {
    let body = std::str::from_utf8(&request.body).map_err(|_| invalid("Saved CLI/TUI repeaters require a UTF-8 body; use http_request_send with body_base64 for binary requests"))?;
    let url = reqwest::Url::parse(&request.uri).map_err(invalid)?;
    let mut raw = format!(
        "{} {}{} {}\r\n",
        request.method,
        url.path(),
        url.query().map(|q| format!("?{}", q)).unwrap_or_default(),
        request.version
    );
    for (name, value) in &request.headers {
        raw.push_str(&format!(
            "{}: {}\r\n",
            name,
            value.to_str().map_err(invalid)?
        ));
    }
    raw.push_str("\r\n");
    raw.push_str(body);
    Ok(raw)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::siv_ui::repeater::{RepeaterParameters, RepeaterState, RepeaterStateSerializable};

    fn state(raw: &str) -> RepeaterState {
        RepeaterState::try_from(RepeaterStateSerializable {
            name: "parser regression".into(),
            request: base64::encode(raw),
            response: None,
            parameters: RepeaterParameters {
                address: "[::1]:8080".into(),
                https: false,
                redirects: false,
                max_redirects: 0,
            },
        })
        .unwrap()
    }

    #[test]
    fn make_reqwest_and_shared_parser_preserve_body_headers_and_connection_target() {
        for newline in ["\n", "\r\n"] {
            let body = "Привет\0\r\n\r\nline\n\nend  \n";
            let raw = format!("POST /p?q=%D1%82 HTTP/1.1{newline}Host: virtual.test{newline}X-Dup: 1{newline}X-Dup: 2{newline}Content-Length: 1{newline}{newline}{body}");
            let parsed = parse_raw(&raw, "http://[::1]:8080/").unwrap();
            let request = state(&raw).make_reqwest().unwrap();
            assert_eq!(request.url().as_str(), parsed.uri);
            assert_eq!(request.url().as_str(), "http://[::1]:8080/p?q=%D1%82");
            assert_eq!(request.body().unwrap().as_bytes().unwrap(), body.as_bytes());
            assert_eq!(request.headers().get_all("x-dup").iter().count(), 2);
            assert_eq!(request.headers()["host"], "virtual.test");
            assert!(!request.headers().contains_key("content-length"));
            assert_eq!(
                parse_raw(&render_raw(&parsed).unwrap(), &parsed.uri)
                    .unwrap()
                    .body,
                body.as_bytes()
            );
        }
    }

    #[test]
    fn legacy_cli_empty_body_without_separator_and_http_versions_still_work() {
        for raw in [
            "GET / HTTP/1.1",
            "GET / HTTP/1.1\r\nHost: virtual.test\r\n",
            "GET / HTTP/2\n",
            "GET / HTTP/1.0\n\n",
        ] {
            assert!(state(raw)
                .make_reqwest()
                .unwrap()
                .body()
                .unwrap()
                .as_bytes()
                .unwrap()
                .is_empty());
        }
        for raw in [
            "",
            "bad request",
            "GET / HTTP/3\n\n",
            "GET / HTTP/1.1\nInvalidHeader\n\n",
        ] {
            assert!(state(raw).make_reqwest().is_err());
        }
        // MCP's raw-edit contract remains strict, unlike the legacy CLI adapter.
        assert!(parse_raw("GET / HTTP/1.1", "http://localhost/").is_err());
    }
}
