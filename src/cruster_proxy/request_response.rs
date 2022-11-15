use std::fmt::format;
use http::HeaderMap;
use hudsucker::{
    hyper::{Body, Request, Response, self},
};
use log::debug;
use tokio::sync::mpsc::Sender;
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use crate::CrusterError;
use crate::ui::ui_storage::HEADER_NAME_COLOR;
use flate2::write::GzDecoder;
use std::io::prelude::*;
use bstr::ByteSlice;


#[derive(Clone, Debug)]
pub(crate) struct HyperRequestWrapper {
    pub(crate) uri: String,
    pub(crate) method: String,
    pub(crate) version: String,
    pub(crate) headers: hyper::HeaderMap,
    pub(crate) body: Vec<u8>
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

    pub fn to_vec_of_spans(&self) -> Vec<Spans<'static>> {
        let mut request_list: Vec<Spans> = Vec::new();
        let tmp: Vec<Span> = vec![
            Span::styled(self.method.clone(), Style::default().add_modifier(Modifier::BOLD)),
            Span::from(" "),
            Span::from(self.get_request_path()),
            Span::from(" "),
            Span::from(format!("{}", self.version)),
        ];
        request_list.push(Spans::from(tmp));

        for (k, v) in self.headers.iter() {
            let mut tmp: Vec<Span> = Vec::new();
            tmp.push(Span::styled(k.to_string(), Style::default().fg(HEADER_NAME_COLOR)));
            tmp.push(Span::from(": ".to_string()));
            tmp.push(Span::from(format!("{}", v.to_str().unwrap())));
            request_list.push(Spans::from(tmp));
        }

        request_list.push(Spans::from(Span::from("")));

        // TODO: requests body hiding
        for line in self.body.to_str_lossy().split("\n") {
            request_list.push(Spans::from(line.to_string()));
        }

        return request_list;
    }

    pub fn to_string(&self) -> String {
        let mut result = format!(
            "{} {} {}\r\n",
            self.method.as_str(),
            self.get_request_path().as_str(),
            self.version.as_str()
        );

        for (k, v) in self.headers.iter() {
            result = format!(
                "{}{}: {}\r\n",
                result,
                k.as_str(),
                v.to_str().unwrap()
            );
        }

        result = format!("{}\r\n{}", result, self.body.to_str_lossy().to_string());

        return result;
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

impl HyperResponseWrapper {
    pub(crate) async fn from_hyper(
            rsp: Response<Body>,
            err_pipe: Option<& Sender<CrusterError>>) -> Result<(Self, Response<Body>), CrusterError> {

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
                            if let Some(err_tx) = err_pipe {
                                let err_send_result = err_tx
                                    .send(CrusterError::UnknownResponseBodyEncoding(
                                        format!("Found unknown encoding for response body: {}", s))
                                    )
                                    .await;
                                if let Err(send_err) = err_send_result {
                                    panic!("Cannot send error about encoding to UI: {}", send_err.to_string());
                                }
                            }
                        }
                    },
                    Err(e) => {
                        if let Some(err_tx) = err_pipe {
                            let err_send_result = err_tx.send(e.into()).await;
                            if let Err(send_err) = err_send_result {
                                panic!("Cannot send error message to UI thread: {}", send_err.to_string());
                            }
                        }
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

    pub fn to_vec_of_spans(&self, reveal_body: bool) -> Result<Vec<Spans<'static>>, CrusterError> {
        let mut response_content: Vec<Spans> = vec![];

        // Status and version, like '200 OK HTTP/2'
        let first_line = Spans::from(vec![
            Span::styled(
                self.status.clone(),
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD)
            ),
            Span::from(" "),
            Span::from(self.version.clone())
        ]);
        response_content.push(first_line);

        // Response Headers
        for (k, v) in &self.headers {
            let header_line = Spans::from(vec![
                Span::styled(
                    k.clone().to_string(),
                    Style::default().fg(HEADER_NAME_COLOR)
                ),
                Span::from(": "),
                Span::from(v.clone().to_str().unwrap().to_string())
            ]);
            response_content.push(header_line)
        }

        // Empty line
        response_content.push(Spans::default());

        // Body
        let body = Spans::from(
            match self.body_compressed {
                BodyCompressedWith::NONE => {
                    match self.body.as_slice().to_str() {
                        Ok(s) => Span::from(s.to_string()),
                        Err(e) => {
                            if self.body.len() > 2 * 1024 * 1024 /* 2 Mb */ && ! reveal_body {
                                Span::styled(
                                    "Response body too large, too see it  select (activate) response square (default key is [s]) ".to_owned() +
                                        "and press [u] to reveal",
                                    Style::default().fg(Color::DarkGray)
                                )
                            }
                            else {
                                Span::from(String::from_utf8_lossy(self.body.as_slice()).to_string())
                            }
                        }
                    }
                },
                BodyCompressedWith::GZIP => {
                    if self.body.len() > 1024 * 1024 /* 1 Mb */ && ! reveal_body {
                        Span::styled(
                            "Response body too large, too see it select (activate) response square (default key is [s])",
                            Style::default().fg(Color::DarkGray)
                        )
                    }
                    else {
                        let writer = Vec::new();
                        let mut decoder = GzDecoder::new(writer);
                        decoder.write_all(self.body.as_slice()).unwrap();
                        Span::from(decoder.finish().unwrap().to_str_lossy().to_string())
                    }
                },
                BodyCompressedWith::DEFLATE => {
                    return Err(CrusterError::UndefinedError("Decoding 'deflate' is not implemented yet".to_string()));
                },
                BodyCompressedWith::BR => {
                    // TODO: remove err when will support 'br'
                    let error_string = "'Brotli' encoding is not implemented yet.";
                    // self.log_error(CrusterError::NotImplementedError(error_string.to_string()));
                    Span::styled(
                        error_string,
                        Style::default().fg(Color::DarkGray)
                    )
                }
            });

        response_content.push(body);
        return Ok(response_content);
    }
}

// ---------------------------------------------------------------------------------------------- //

pub(crate) enum CrusterWrapper {
    Request(HyperRequestWrapper),
    Response(HyperResponseWrapper)
}
