use base64;
use std::{io::Write, sync::mpsc::Receiver};
use serde_json as json;
use std::{collections::HashMap, fs};
use serde::{Serialize, Deserialize};
use super::{RequestResponsePair, HTTPStorage};
use crate::{cruster_proxy::request_response::{HyperRequestWrapper, HyperResponseWrapper}, utils::CrusterError};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct HeaderValue {
    encoding: String,
    value: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct SerializableHTTPRequest {
    method: String,
    scheme: String,
    host: String,
    path: String,
    query: Option<String>,
    version: String,
    headers: HashMap<String, HeaderValue>,
    body: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct SerializableHTTPResponse {
    status: String,
    version: String,
    headers: HashMap<String, HeaderValue>,
    body: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(super) struct SerializableProxyData {
    index: usize,
    request: SerializableHTTPRequest,
    response: Option<SerializableHTTPResponse>
}

impl From<&HyperRequestWrapper> for SerializableHTTPRequest {
    fn from(request: &HyperRequestWrapper) -> Self {
        let host = request.get_hostname();
        let (path, query) = if let Ok(pth) = request.get_request_path_without_query() {
            let qr = request.get_query();
            (pth, qr)
        }
        else {
            (request.get_request_path(), None)
        };

        let headers: HashMap<String, HeaderValue> = request.headers
            .iter()
            .map(|(k, v)| {
                let key = k.to_string();
                let value = if let Ok(decoded_value) = v.to_str() {
                    HeaderValue::new("utf-8", decoded_value)
                }
                else {
                    HeaderValue::new("base64", base64::encode(v.as_bytes()))
                };

                (key, value)
            })
            .collect();
        
        let body = if request.body.is_empty() {
            None
        }
        else {
            Some(base64::encode(request.body.as_slice()))
        };

        SerializableHTTPRequest {
            method: request.method.clone(),
            scheme: request.get_scheme(),
            host,
            path,
            query,
            version: request.version.clone(),
            headers,
            body
        }
    }
}

impl From<&HyperResponseWrapper> for SerializableHTTPResponse {
    fn from(response: &HyperResponseWrapper) -> Self {
        let headers: HashMap<String, HeaderValue> = response.headers
            .iter()
            .map(|(k, v)| {
                let key = k.to_string();
                let value = if let Ok(decoded_value) = v.to_str() {
                    HeaderValue::new("utf-8", decoded_value)
                }
                else {
                    HeaderValue::new("base64", base64::encode(v.as_bytes()))
                };

                (key, value)
            })
            .collect();

            let body = if response.body.is_empty() {
                None
            }
            else {
                Some(base64::encode(response.body.as_slice()))
            };

            SerializableHTTPResponse {
                status: response.status.clone(),
                version: response.version.clone(),
                headers,
                body
            }
    }
}

impl TryFrom<&RequestResponsePair> for SerializableProxyData {
    type Error = CrusterError;
    fn try_from(pair: &RequestResponsePair) -> Result<Self, Self::Error> {
        return if pair.request.is_none() {
            Err(CrusterError::EmptyRequest(format!("Could not store record with id {} because  of empty request.", pair.index)))
        }
        else {
            Ok(
                Self {
                    index: pair.index.clone(),
                    request: SerializableHTTPRequest::from(pair.request.as_ref().unwrap()),
                    response: if let Some(rsp) = &pair.response {
                        Some(SerializableHTTPResponse::from(rsp))
                    }
                    else {
                        None
                    }
                }
            )
        };
    }
}

impl HeaderValue {
    fn new<T: ToString, U: ToString>(encoding: T, value: U) -> Self {
        HeaderValue { encoding: encoding.to_string(), value: value.to_string() }
    }
}

impl HTTPStorage {
    // 'Sentinel' used in a case when this method called in separate thread, in one-threaded case it can be None
    // It's needed to interrupt thread after some time expired, because rust threads cannot interrupr 
    // https://internals.rust-lang.org/t/thread-cancel-support/3056
    pub(crate) fn store(&self, path: &str, sentinel: Option<Receiver<usize>>) -> Result<(), CrusterError> {
        let mut fout = fs::OpenOptions::new().write(true).open(path)?;
        for pair in &self.storage {
            let serializable_record = SerializableProxyData::try_from(pair)?;
            let jsn = json::to_string(&serializable_record)?;
            let _bytes_written = fout.write(jsn.as_bytes())?;
            let _one_byte_written = fout.write("\n".as_bytes())?;

            if let Some(rx) = &sentinel {
                if let Ok(max_duration) = rx.try_recv() {
                    return Err(CrusterError::JobDurateTooLongError(
                        format!("Process of storing proxy data was interrupted, it was running longer that {} seconds.", max_duration)
                    ));
                }
            }
        }

        Ok(())
    }
}