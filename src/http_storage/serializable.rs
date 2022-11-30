use base64;
use bstr::ByteSlice;
use std::io::Write;
use serde_json as json;
use std::{collections::HashMap, fs};
use serde::{Serialize, Deserialize};
use super::{RequestResponsePair, HTTPStorage};
use crate::{cruster_proxy::request_response::{HyperRequestWrapper, HyperResponseWrapper}, utils::CrusterError};


#[derive(Serialize, Deserialize, Debug, Clone)]
struct SerializableHTTPRequest {
    method: String,
    host: String,
    path: String,
    query: Option<String>,
    version: String,
    headers: HashMap<String, String>,
    body: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct SerializableHTTPResponse {
    status: String,
    version: String,
    headers: HashMap<String, String>,
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
        let host = request.get_host();
        let (path, query) = if let Ok(pth) = request.get_request_path_without_query() {
            let qr = request.get_query();
            (pth, qr)
        }
        else {
            (request.get_request_path(), None)
        };

        let headers: HashMap<String, String> = request.headers
            .iter()
            .map(|(k, v)| {
                (k.to_string(), v.as_bytes().to_str_lossy().to_string())
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
        let headers: HashMap<String, String> = response.headers
            .iter()
            .map(|(k, v)| {
                // TODO: make format for value like UNDECODED![base64_encoded_header_value]
                (k.to_string(), v.as_bytes().to_str_lossy().to_string())
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

impl HTTPStorage {
    pub(crate) fn store(&self, path: &str) -> Result<(), CrusterError> {
        let mut fout = fs::OpenOptions::new().write(true).open(path)?;
        for pair in &self.storage {
            let serializable_record = SerializableProxyData::try_from(pair)?;
            let jsn = json::to_string(&serializable_record)?;
            let _bytes_written = fout.write(jsn.as_bytes())?;
            let _one_byte_written = fout.write("\n".as_bytes())?;
        }

        Ok(())
    }
}