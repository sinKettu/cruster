use http::HeaderMap;
use regex::bytes::{Captures, Regex as ByteRegex};
use crate::{audit::actions::ExtractionMode, cruster_proxy::request_response::{HyperRequestWrapper, HyperResponseWrapper}};


fn get_capture(cap: Captures, line: &[u8], mode: &ExtractionMode) -> Vec<u8> {
    match mode {
        ExtractionMode::MATCH => {
            return cap.get(0).unwrap().as_bytes().to_owned();
        },
        ExtractionMode::LINE => {
            return line.to_owned();
        },
        ExtractionMode::GROUP(gname) => {
            match cap.name(gname) {
                Some(group) => {
                    return group.as_bytes().to_owned();
                },
                None => {
                    return vec![]
                }
            }
        }
    }
}

fn get_capture_from_header_map(re: &ByteRegex, headers: &HeaderMap, mode: &ExtractionMode) -> Option<Vec<u8>> {
    for k in headers.keys() {
        let v = headers
            .get_all(k)
            .iter()
            .flat_map(|hv| { [hv.as_bytes(), "; ".as_bytes()] })
            .flatten()
            .map(|c| { *c })
            .collect::<Vec<u8>>();

        let header_line = [
            k.as_str().as_bytes(),
            ": ".as_bytes(),
            &v[0..v.len() - 2],
            "\r\n".as_bytes()
        ]
            .into_iter()
            .flatten()
            .map(|c| { *c })
            .collect::<Vec<u8>>();

        if let Some(cap) = re.captures(&header_line) {
            return Some(get_capture(cap, &header_line, mode));
        }
    }

    None
}

fn get_capture_from_body(re: &ByteRegex, body: &[u8], mode: &ExtractionMode) -> Option<Vec<u8>> {
    for body_line in body.split(|c| { *c == b'c' }) {
        if let Some(cap) = re.captures(&body_line) {
            return Some(get_capture(cap, body_line, mode));
        }
    }

    None
}

pub(crate) trait ExtractFromHTTPPartByRegex {
    fn extract_from_first_line(&self, re: &ByteRegex, mode: &ExtractionMode) -> Option<Vec<u8>>;
    fn extract_from_headers(&self, re: &ByteRegex, mode: &ExtractionMode) -> Option<Vec<u8>>;
    fn extract_from_body(&self, re: &ByteRegex, mode: &ExtractionMode) -> Option<Vec<u8>>;
    fn extract(&self, re:&ByteRegex, mode: &ExtractionMode) -> Option<Vec<u8>>;
}

impl ExtractFromHTTPPartByRegex for HyperRequestWrapper {
    fn extract_from_first_line(&self, re: &ByteRegex, mode: &ExtractionMode) -> Option<Vec<u8>> {
        let path = self.uri.split("/").skip(3).collect::<Vec<&str>>().join("/");
        let first_line = [
            self.method.as_bytes(),
            " /".as_bytes(),
            path.as_bytes(),
            " ".as_bytes(),
            self.version.as_bytes(),
            "\r\n".as_bytes()
        ]
            .into_iter()
            .flatten()
            .map(|c| { *c })
            .collect::<Vec<u8>>();
        
        let cap = re.captures(&first_line);
        
        if let Some(cap) = cap {
            Some(get_capture(cap, &first_line, mode))
        }
        else {
            None
        }
    }

    fn extract_from_headers(&self, re: &ByteRegex, mode: &ExtractionMode) -> Option<Vec<u8>> {
        get_capture_from_header_map(re, &self.headers, mode)
    }

    fn extract_from_body(&self, re: &ByteRegex, mode: &ExtractionMode) -> Option<Vec<u8>> {
        get_capture_from_body(re, &self.body, mode)
    }

    fn extract(&self, re:&ByteRegex, mode: &ExtractionMode) -> Option<Vec<u8>> {
        let from_first_line = self.extract_from_first_line(re, mode);
        if from_first_line.is_some() {
            return from_first_line;
        }

        let from_headers = self.extract_from_headers(re, mode);
        if from_headers.is_some() {
            return from_headers;
        }

        let from_body = self.extract_from_body(re, mode);
        if from_body.is_some() {
            return from_body;
        }

        None
    }
}

impl ExtractFromHTTPPartByRegex for HyperResponseWrapper {
    fn extract_from_first_line(&self, re: &ByteRegex, mode: &ExtractionMode) -> Option<Vec<u8>> {
        let first_line = [
            self.status.as_bytes(),
            " ".as_bytes(),
            self.version.as_bytes(),
            "\r\n".as_bytes()
        ]
            .into_iter()
            .flatten()
            .map(|c| { *c })
            .collect::<Vec<u8>>();

        let cap = re.captures(&first_line);
        
        if let Some(cap) = cap {
            Some(get_capture(cap, &first_line, mode))
        }
        else {
            None
        }
    }

    fn extract_from_headers(&self, re: &ByteRegex, mode: &ExtractionMode) -> Option<Vec<u8>> {
        get_capture_from_header_map(re, &self.headers, mode)
    }

    fn extract_from_body(&self, re: &ByteRegex, mode: &ExtractionMode) -> Option<Vec<u8>> {
        get_capture_from_body(re, &self.body, mode)
    }

    fn extract(&self, re:&ByteRegex, mode: &ExtractionMode) -> Option<Vec<u8>> {
        let from_first_line = self.extract_from_first_line(re, mode);
        if from_first_line.is_some() {
            return from_first_line;
        }

        let from_headers = self.extract_from_headers(re, mode);
        if from_headers.is_some() {
            return from_headers;
        }

        let from_body = self.extract_from_body(re, mode);
        if from_body.is_some() {
            return from_body;
        }

        None
    }
}
