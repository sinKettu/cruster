#[cfg(test)]
use std::fs;

use http::{header::HeaderName, HeaderMap, HeaderValue};
use rmcp::{handler::server::wrapper::Json, schemars, ErrorData as McpError};
use serde::Deserialize;

use super::{internal_error, CrusterMcpServer, DEFAULT_PAGE_SIZE, MAX_PAGE_SIZE};
use crate::{
    cli::repeater as shared,
    cruster_proxy::request_response::{HyperRequestWrapper, HyperResponseWrapper},
    http_storage::serializable::{SerializableHTTPRequest, SerializableHTTPResponse},
    siv_ui::repeater::{RepeaterParameters, RepeaterStateSerializable},
};

fn invalid(error: impl ToString) -> McpError {
    McpError::invalid_params(error.to_string(), None)
}

fn operation_error(error: crate::cli::CrusterCLIError) -> McpError {
    internal_error(Into::<String>::into(error))
}

fn selection_error(error: crate::cli::CrusterCLIError) -> McpError {
    invalid(Into::<String>::into(error))
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(super) struct RequestHeader {
    name: String,
    value: String,
}

#[derive(Debug, Default, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(super) struct RequestEdit {
    /// Replace the complete raw request; the connection target is preserved unless url is supplied.
    raw_request: Option<String>,
    method: Option<String>,
    /// Absolute HTTP(S) URL, including the connection host and optional port. Host header is independent.
    url: Option<String>,
    path: Option<String>,
    /// Query without '?'; an empty string removes the query.
    query: Option<String>,
    version: Option<String>,
    /// Replace all headers. Repeated names are preserved. An empty list removes all headers.
    headers: Option<Vec<RequestHeader>>,
    /// Replace all values of each named header; repeated names in this list are preserved.
    set_headers: Option<Vec<RequestHeader>>,
    remove_headers: Option<Vec<String>>,
    /// UTF-8 body. An empty string clears the body. Mutually exclusive with body_base64.
    body: Option<String>,
    /// Exact binary body in Base64. Mutually exclusive with body.
    body_base64: Option<String>,
}

impl RequestEdit {
    pub(super) fn apply(self, request: &mut HyperRequestWrapper) -> Result<(), McpError> {
        if self.body.is_some() && self.body_base64.is_some() {
            return Err(invalid("Specify body or body_base64, not both"));
        }
        if let Some(raw) = self.raw_request {
            // A URL override can supply the target for a newly authored raw request.
            *request = parse_raw(&raw, self.url.as_deref().unwrap_or(&request.uri))?;
        }
        if let Some(method) = self.method {
            request.method = method;
        }
        if let Some(version) = self.version {
            request.version = version;
        }
        let mut url =
            reqwest::Url::parse(self.url.as_deref().unwrap_or(&request.uri)).map_err(invalid)?;
        if let Some(path) = self.path {
            url.set_path(&path);
        }
        if let Some(query) = self.query {
            url.set_query((!query.is_empty()).then_some(query.as_str()));
        }
        request.uri = url.to_string();
        if let Some(headers) = self.headers {
            request.headers = HeaderMap::new();
            append_headers(&mut request.headers, headers)?;
        }
        if let Some(names) = self.remove_headers {
            for name in names {
                request
                    .headers
                    .remove(name.parse::<HeaderName>().map_err(invalid)?);
            }
        }
        if let Some(headers) = self.set_headers {
            for header in &headers {
                request
                    .headers
                    .remove(header.name.parse::<HeaderName>().map_err(invalid)?);
            }
            append_headers(&mut request.headers, headers)?;
        }
        if let Some(body) = self.body {
            request.body = body.into_bytes();
        }
        if let Some(body) = self.body_base64 {
            request.body = base64::decode(body).map_err(invalid)?;
        }
        validate(request)?;
        // The shared sender computes the final body length.
        request.headers.remove("content-length");
        Ok(())
    }
}

fn append_headers(headers: &mut HeaderMap, values: Vec<RequestHeader>) -> Result<(), McpError> {
    for header in values {
        headers.append(
            header.name.parse::<HeaderName>().map_err(invalid)?,
            HeaderValue::from_str(&header.value).map_err(invalid)?,
        );
    }
    Ok(())
}

fn validate(request: &HyperRequestWrapper) -> Result<(), McpError> {
    shared::parsing::validate(request).map_err(invalid)
}

fn parse_raw(raw: &str, target: &str) -> Result<HyperRequestWrapper, McpError> {
    shared::parsing::parse_raw(raw, target).map_err(invalid)
}

fn render_raw(request: &HyperRequestWrapper) -> Result<String, McpError> {
    shared::parsing::render_raw(request).map_err(invalid)
}

use shared::parsing::split_raw;

#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(super) struct RepeaterListInput {
    #[serde(default)]
    offset: usize,
    limit: Option<usize>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(super) struct RepeaterGetInput {
    /// One-based position, compatible with CLI numbering. Deletion renumbers later entries.
    id: Option<usize>,
    /// Exact saved name instead of id. If names repeat, selects the first, like CLI.
    name: Option<String>,
    #[serde(default = "super::default_true")]
    include_bodies: bool,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
enum ContentPart {
    Request,
    Response,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(super) struct RepeaterContentInput {
    id: Option<usize>,
    name: Option<String>,
    part: ContentPart,
    /// Byte offset, not a character offset.
    #[serde(default)]
    offset: usize,
    /// Maximum bytes per chunk. Automatically reduced to fit the configured output limit.
    limit: Option<usize>,
}

#[derive(Debug, Default, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(super) struct RepeaterChanges {
    name: Option<String>,
    edits: Option<RequestEdit>,
    /// Connection host or IP, with optional port; independent of the Host header.
    address: Option<String>,
    https: Option<bool>,
    redirects: Option<bool>,
    max_redirects: Option<usize>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(super) struct RepeaterCreateInput {
    /// History pair to copy. Omit when supplying raw_request, address and https.
    history_id: Option<usize>,
    raw_request: Option<String>,
    #[serde(default)]
    changes: RepeaterChanges,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(super) struct RepeaterEditInput {
    id: Option<usize>,
    name: Option<String>,
    changes: RepeaterChanges,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(super) struct RepeaterSendInput {
    id: Option<usize>,
    name: Option<String>,
    /// Applied and saved before sending; omitted fields keep their current values.
    changes: Option<RepeaterChanges>,
    #[serde(default = "super::default_true")]
    include_bodies: bool,
}

impl RepeaterStateSerializable {
    fn apply(&mut self, changes: RepeaterChanges) -> Result<(), McpError> {
        shared::edit::RepeaterEditSettings {
            name: changes.name,
            address: changes.address,
            https: changes.https,
            redirects: changes.redirects,
            max_redirects: changes.max_redirects,
            ..Default::default()
        }
        .apply(self);
        let mut request = self.parsed_request().map_err(selection_error)?;
        if let Some(edits) = changes.edits {
            edits.apply(&mut request)?;
        }
        let url = reqwest::Url::parse(&request.uri).map_err(invalid)?;
        self.parameters.address = url
            .as_str()
            .split('/')
            .nth(2)
            .unwrap_or_default()
            .to_string();
        self.parameters.https = url.scheme() == "https";
        self.request = base64::encode(render_raw(&request)?);
        self.response = None;
        Ok(())
    }

    fn view(&self, id: usize, bodies: bool) -> Result<serde_json::Value, McpError> {
        let raw = self.raw().map_err(operation_error)?;
        let request = if bodies {
            raw
        } else {
            split_raw(&raw).map_or(raw.clone(), |(head, _)| format!("{}\r\n\r\n", head))
        };
        let response = self
            .response
            .as_ref()
            .map(|encoded| -> Result<String, McpError> {
                let raw = String::from_utf8(base64::decode(encoded).map_err(internal_error)?)
                    .map_err(internal_error)?;
                Ok(if bodies {
                    raw
                } else {
                    split_raw(&raw).map_or(raw.clone(), |(head, _)| format!("{}\r\n\r\n", head))
                })
            })
            .transpose()?;
        Ok(
            serde_json::json!({"id": id, "name": self.name, "request": request, "response": response, "parameters": self.parameters}),
        )
    }
}

impl CrusterMcpServer {
    fn require_repeater_write(&self) -> Result<(), McpError> {
        if !self.settings.allow_dangerous_tools {
            return Err(invalid(
                "Dangerous MCP tools are disabled; restart with --allow-dangerous-tools",
            ));
        }
        Ok(())
    }

    fn read_repeaters(&self) -> Result<Vec<RepeaterStateSerializable>, McpError> {
        shared::read_repeaters(&self.project.join("repeater.jsonl")).map_err(operation_error)
    }

    fn write_repeaters(&self, records: &[RepeaterStateSerializable]) -> Result<(), McpError> {
        shared::write_repeaters(&self.project.join("repeater.jsonl"), records)
            .map_err(operation_error)
    }

    fn repeater_index(records: &[RepeaterStateSerializable], id: usize) -> Result<usize, McpError> {
        Self::select_repeater(records, Some(id), None)
    }

    fn select_repeater(
        records: &[RepeaterStateSerializable],
        id: Option<usize>,
        name: Option<&str>,
    ) -> Result<usize, McpError> {
        shared::operations::select_repeater(records, id, name).map_err(selection_error)
    }

    pub(super) fn list_repeaters(
        &self,
        input: RepeaterListInput,
    ) -> Result<Json<serde_json::Value>, McpError> {
        let records = self.read_repeaters()?;
        let limit = input
            .limit
            .unwrap_or(DEFAULT_PAGE_SIZE)
            .clamp(1, MAX_PAGE_SIZE);
        let items: Vec<_> = records.iter().enumerate().skip(input.offset).take(limit).map(|(i, r)| serde_json::json!({"id": i + 1, "name": r.name, "parameters": r.parameters, "has_response": r.response.is_some()})).collect();
        let end = input.offset.saturating_add(items.len());
        self.bounded_json(&serde_json::json!({"items": items, "next_offset": (end < records.len()).then_some(end)}))
    }

    pub(super) fn get_repeater(
        &self,
        input: RepeaterGetInput,
    ) -> Result<Json<serde_json::Value>, McpError> {
        let records = self.read_repeaters()?;
        let index = Self::select_repeater(&records, input.id, input.name.as_deref())?;
        self.bounded_json(&records[index].view(index + 1, input.include_bodies)?)
    }

    pub(super) fn get_repeater_content(
        &self,
        input: RepeaterContentInput,
    ) -> Result<Json<serde_json::Value>, McpError> {
        let records = self.read_repeaters()?;
        let index = Self::select_repeater(&records, input.id, input.name.as_deref())?;
        let encoded = match input.part {
            ContentPart::Request => &records[index].request,
            ContentPart::Response => records[index]
                .response
                .as_ref()
                .ok_or_else(|| invalid("Repeater has no saved response"))?,
        };
        let bytes = base64::decode(encoded).map_err(internal_error)?;
        if input.offset > bytes.len() {
            return Err(invalid("Offset exceeds content size"));
        }
        let mut length = input
            .limit
            .unwrap_or(16384)
            .clamp(1, 65536)
            .min(bytes.len() - input.offset);
        loop {
            let end = input.offset + length;
            let chunk = &bytes[input.offset..end];
            let value = serde_json::json!({"id": index + 1, "offset": input.offset, "total_bytes": bytes.len(), "data_base64": base64::encode(chunk), "text": std::str::from_utf8(chunk).ok(), "next_offset": (end < bytes.len()).then_some(end)});
            if serde_json::to_vec(&value).map_err(internal_error)?.len()
                <= self.settings.max_result_bytes
                || length <= 1
            {
                return self.bounded_json(&value);
            }
            length /= 2;
        }
    }

    pub(super) fn create_repeater(
        &self,
        input: RepeaterCreateInput,
    ) -> Result<Json<serde_json::Value>, McpError> {
        self.require_repeater_write()?;
        let records = self.read_repeaters()?;
        let id = records.len() + 1;
        let mut record = if let Some(history_id) = input.history_id {
            let pair = self
                .read_history()?
                .into_iter()
                .find(|p| p.index == history_id)
                .ok_or_else(|| invalid(format!("HTTP pair {} was not found", history_id)))?;
            let request: HyperRequestWrapper = pair.request.try_into().map_err(internal_error)?;
            validate(&request)?;
            shared::operations::from_history(&request, None, format!("Repeater #{}", id), 5)
                .map_err(selection_error)?
        } else {
            if input.raw_request.is_none()
                || input.changes.address.is_none()
                || input.changes.https.is_none()
            {
                return Err(invalid(
                    "Supply history_id or raw_request with changes.address and changes.https",
                ));
            }
            RepeaterStateSerializable {
                name: format!("Repeater #{}", id),
                request: String::new(),
                response: None,
                parameters: RepeaterParameters {
                    address: String::new(),
                    https: false,
                    redirects: true,
                    max_redirects: 5,
                },
            }
        };
        if let Some(raw) = input.raw_request {
            record.request = base64::encode(raw);
        }
        record.apply(input.changes)?;
        let result = self.bounded_json(&record.view(id, true)?)?;
        shared::update_repeaters(&self.project.join("repeater.jsonl"), &record, usize::MAX)
            .map_err(operation_error)?;
        Ok(result)
    }

    pub(super) fn update_repeater(
        &self,
        input: RepeaterEditInput,
    ) -> Result<Json<serde_json::Value>, McpError> {
        self.require_repeater_write()?;
        let mut records = self.read_repeaters()?;
        let index = Self::select_repeater(&records, input.id, input.name.as_deref())?;
        records[index].apply(input.changes)?;
        let result = self.bounded_json(&records[index].view(index + 1, true)?)?;
        shared::update_repeaters(&self.project.join("repeater.jsonl"), &records[index], index)
            .map_err(operation_error)?;
        Ok(result)
    }

    pub(super) fn delete_repeater(&self, id: usize) -> Result<Json<serde_json::Value>, McpError> {
        self.require_repeater_write()?;
        let mut records = self.read_repeaters()?;
        let index = Self::repeater_index(&records, id)?;
        records.remove(index);
        self.write_repeaters(&records)?;
        self.bounded_json(&serde_json::json!({"deleted_id": id, "remaining": records.len()}))
    }

    pub(super) fn delete_repeater_selected(
        &self,
        input: RepeaterGetInput,
    ) -> Result<Json<serde_json::Value>, McpError> {
        self.require_repeater_write()?;
        let records = self.read_repeaters()?;
        let index = Self::select_repeater(&records, input.id, input.name.as_deref())?;
        self.delete_repeater(index + 1)
    }

    pub(super) async fn send_repeater_request(
        &self,
        request: &HyperRequestWrapper,
        max_redirects: usize,
    ) -> Result<(HyperResponseWrapper, bool), McpError> {
        self.require_repeater_write()?;
        validate(request)?;
        shared::operations::send_request(
            request,
            max_redirects,
            Some(std::time::Duration::from_secs(
                self.settings.request_timeout_seconds,
            )),
        )
        .await
        .map_err(operation_error)
    }

    pub(super) fn send_result(
        &self,
        request: &HyperRequestWrapper,
        response: &HyperResponseWrapper,
        reached: bool,
        bodies: bool,
    ) -> Result<Json<serde_json::Value>, McpError> {
        let mut request = SerializableHTTPRequest::from(request);
        let mut structured_response = SerializableHTTPResponse::from(response);
        let raw_response = if bodies {
            Some(response.to_string())
        } else {
            None
        };
        if !bodies {
            request.body = None;
            structured_response.body = None;
        }
        self.bounded_json(&serde_json::json!({"request": request, "response": raw_response, "structured_response": structured_response, "redirect_limit_reached": reached}))
    }

    pub(super) async fn execute_repeater(
        &self,
        input: RepeaterSendInput,
    ) -> Result<Json<serde_json::Value>, McpError> {
        self.require_repeater_write()?;
        let mut records = self.read_repeaters()?;
        let index = Self::select_repeater(&records, input.id, input.name.as_deref())?;
        let id = index + 1;
        if let Some(changes) = input.changes {
            records[index].apply(changes)?;
        }
        let request = records[index].parsed_request().map_err(selection_error)?;
        let (response, reached) = shared::operations::execute_repeater(
            &self.project.join("repeater.jsonl"),
            &mut records[index],
            index,
            &request,
            Some(std::time::Duration::from_secs(
                self.settings.request_timeout_seconds,
            )),
        )
        .await
        .map_err(operation_error)?;
        // Persist before checking the output limit: retrying get never resends a request.
        let mut result = self.send_result(&request, &response, reached, input.include_bodies).map_err(|e| invalid(format!("Request was sent and response saved in repeater {}. Use repeater_get with include_bodies=false. Result error: {}", id, e)))?;
        result.0["repeater_id"] = serde_json::json!(id);
        self.bounded_json(&result.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{AuditConfig, McpConfig};
    use rmcp::handler::server::wrapper::Parameters;
    use serde::de::DeserializeOwned;
    use serde_json::json;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    #[tokio::test]
    async fn cli_name_selection_supports_all_repeater_operations() {
        let project = Project::new(true);
        project.capture();
        project.create();
        assert_eq!(
            project
                .0
                .get_repeater(input(json!({"name":"agent test"})))
                .unwrap()
                .0["id"],
            1
        );
        assert!(project
            .0
            .get_repeater(input(json!({"id":1,"name":"agent test"})))
            .is_err());
        assert!(project.0.get_repeater(input(json!({}))).is_err());
        project
            .0
            .update_repeater(input(
                json!({"name":"agent test","changes":{"name":"renamed"}}),
            ))
            .unwrap();
        let (address, received) = local_server(b"response body".to_vec()).await;
        let sent = project
            .0
            .execute_repeater(input(
                json!({"name":"renamed","changes":{"address":address}}),
            ))
            .await
            .unwrap()
            .0;
        assert_eq!(sent["repeater_id"], 1);
        received.await.unwrap();
        let headers = project
            .0
            .get_repeater(input(json!({"name":"renamed","include_bodies":false})))
            .unwrap()
            .0;
        assert!(headers["response"]
            .as_str()
            .unwrap()
            .starts_with("HTTP/1.1 200"));
        assert!(!headers["response"]
            .as_str()
            .unwrap()
            .contains("response body"));
        let content = project
            .0
            .get_repeater_content(input(json!({"name":"renamed","part":"response"})))
            .unwrap()
            .0;
        assert!(content["text"].as_str().unwrap().contains("response body"));
        project
            .0
            .delete_repeater_selected(input(json!({"name":"renamed"})))
            .unwrap();
        assert!(project.0.read_repeaters().unwrap().is_empty());
    }

    fn input<T: DeserializeOwned>(value: serde_json::Value) -> T {
        serde_json::from_value(value).unwrap()
    }

    #[test]
    fn cli_add_edit_and_mcp_share_persistence_and_first_name_selection() {
        let project = Project::new(true);
        project.capture();
        let path = project.0.project.join("repeater.jsonl");
        shared::add::exec(
            &shared::add::RepeaterAddSettings { index: 42 },
            project.0.history_path().to_str().unwrap(),
            path.to_str().unwrap(),
        )
        .unwrap();
        let created = project.0.get_repeater(input(json!({"id":1}))).unwrap().0;
        assert_eq!(created["name"], "Repeater #0");
        assert_eq!(created["parameters"]["max_redirects"], 10);
        project.create();
        for id in [1, 2] {
            shared::edit::execute(
                &shared::edit::RepeaterEditSettings {
                    number_to_get: Some(id),
                    name: Some("duplicate".into()),
                    ..Default::default()
                },
                path.to_str().unwrap(),
            )
            .unwrap();
        }
        shared::edit::execute(
            &shared::edit::RepeaterEditSettings {
                name_to_get: Some("duplicate".into()),
                address: Some("localhost:9000".into()),
                redirects: Some(false),
                max_redirects: Some(0),
                ..Default::default()
            },
            path.to_str().unwrap(),
        )
        .unwrap();
        let first = project
            .0
            .get_repeater(input(json!({"name":"duplicate"})))
            .unwrap()
            .0;
        assert_eq!(first["id"], 1);
        assert_eq!(first["parameters"]["address"], "localhost:9000");
        assert_eq!(first["parameters"]["redirects"], false);
        assert_eq!(first["parameters"]["max_redirects"], 0);
        assert_ne!(
            project.0.get_repeater(input(json!({"id":2}))).unwrap().0["parameters"]["address"],
            "localhost:9000"
        );
        project
            .0
            .update_repeater(input(json!({"id":2,"changes":{"name":"MCP → CLI"}})))
            .unwrap();
        let states = shared::RepeaterIterator::new(path.to_str().unwrap())
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        assert_eq!(states[1].name, "MCP → CLI");
    }

    #[tokio::test]
    async fn cli_send_saves_response_for_mcp_and_preserves_utf8_duplicate_headers_and_body() {
        let project = Project::new(true);
        let body = "Привет\r\n\r\nlast\n\n  \n";
        let (address, received) = local_server("Ответ".as_bytes().to_vec()).await;
        project.0.create_repeater(input(json!({"raw_request":format!("POST /cli HTTP/1.1\nHost: virtual.test\nX-Dup: one\nX-Dup: two\nContent-Length: 1\n\n{body}"),"changes":{"address":address,"https":false}}))).unwrap();
        let path = project.0.project.join("repeater.jsonl");
        shared::exec::execute(
            &shared::exec::RepeaterExecSettings {
                name: None,
                number: Some(1),
                force: true,
                no_body: true,
            },
            path.to_str().unwrap(),
            "unused-editor",
        )
        .await
        .unwrap();
        let wire = received.await.unwrap();
        assert!(wire.ends_with(body.as_bytes()));
        let wire = String::from_utf8(wire).unwrap();
        assert!(wire.contains("x-dup: one\r\n"));
        assert!(wire.contains("x-dup: two\r\n"));
        assert!(
            project.0.get_repeater(input(json!({"id":1}))).unwrap().0["response"]
                .as_str()
                .unwrap()
                .ends_with("Ответ")
        );
    }

    #[tokio::test]
    async fn failed_send_saves_edits_and_clears_stale_response_for_cli_and_mcp() {
        let project = Project::new(true);
        project.capture();
        project.create();
        let path = project.0.project.join("repeater.jsonl");
        for cli in [false, true] {
            let mut records = project.0.read_repeaters().unwrap();
            records[0].response = Some(base64::encode("stale response"));
            project.0.write_repeaters(&records).unwrap();
            if cli {
                assert!(shared::exec::execute(
                    &shared::exec::RepeaterExecSettings {
                        name: None,
                        number: Some(1),
                        force: true,
                        no_body: true
                    },
                    path.to_str().unwrap(),
                    "unused-editor"
                )
                .await
                .is_err());
            } else {
                assert!(project.0.execute_repeater(input(json!({"id":1,"changes":{"name":"saved before send","address":"127.0.0.1:0"}}))).await.is_err());
            }
            let saved = project.0.get_repeater(input(json!({"id":1}))).unwrap().0;
            assert!(saved["response"].is_null());
            assert_eq!(saved["name"], "saved before send");
        }
    }

    #[test]
    fn fallible_iterator_and_atomic_updates_handle_blank_and_corrupt_lines() {
        let project = Project::new(true);
        project.capture();
        project.create();
        let path = project.0.project.join("repeater.jsonl");
        let original = fs::read_to_string(&path).unwrap();
        fs::write(&path, format!("\n \n{original}\n{original}")).unwrap();
        shared::edit::execute(
            &shared::edit::RepeaterEditSettings {
                number_to_get: Some(2),
                name: Some("second".into()),
                ..Default::default()
            },
            path.to_str().unwrap(),
        )
        .unwrap();
        assert_eq!(
            project.0.get_repeater(input(json!({"id":1}))).unwrap().0["name"],
            "agent test"
        );
        assert_eq!(
            project.0.get_repeater(input(json!({"id":2}))).unwrap().0["name"],
            "second"
        );
        let record = project.0.read_repeaters().unwrap().remove(0);
        let broken = format!("{original}not JSON\n");
        fs::write(&path, &broken).unwrap();
        let mut iter = shared::RepeaterIterator::new(path.to_str().unwrap()).unwrap();
        assert!(iter.next().unwrap().is_ok());
        assert!(iter.next().unwrap().is_err());
        assert!(iter.next().is_none());
        assert!(shared::update_repeaters(&path, &record, 0).is_err());
        assert_eq!(fs::read_to_string(&path).unwrap(), broken);
        assert!(project.0.get_repeater(input(json!({"id":1}))).is_err());
        for response in ["!".to_owned(), base64::encode([0xff])] {
            let mut corrupt = record.clone();
            corrupt.response = Some(response);
            project.0.write_repeaters(&[corrupt]).unwrap();
            assert!(shared::RepeaterIterator::new(path.to_str().unwrap())
                .unwrap()
                .next()
                .unwrap()
                .is_err());
        }
        let directory_target = project.0.project.join("directory");
        fs::create_dir(&directory_target).unwrap();
        assert!(shared::write_repeaters(&directory_target, &[record]).is_err());
        assert!(directory_target.is_dir());
        assert!(!fs::read_dir(project.0.project.as_ref())
            .unwrap()
            .any(|entry| entry
                .unwrap()
                .file_name()
                .to_string_lossy()
                .ends_with(".tmp")));
    }

    #[tokio::test]
    async fn one_shot_binary_body_is_exact_and_never_persisted_as_saved_repeater() {
        let project = Project::new(true);
        project.capture();
        let (address, received) = local_server(b"done".to_vec()).await;
        let bytes = [0, 255, 128, 13, 10, 13, 10];
        project.0.http_request_send(Parameters(input(json!({"id":42,"edits":{"url":format!("http://{address}/binary"),"body_base64":base64::encode(bytes)}})))).await.unwrap();
        assert!(received.await.unwrap().ends_with(&bytes));
        assert!(!project.0.project.join("repeater.jsonl").exists());
        assert!(project
            .0
            .create_repeater(input(
                json!({"history_id":42,"changes":{"edits":{"body_base64":base64::encode(bytes)}}})
            ))
            .is_err());
        assert!(!project.0.project.join("repeater.jsonl").exists());
    }

    #[tokio::test]
    async fn timeout_clears_stale_response_and_keeps_saved_edits() {
        let project = Project::new(true);
        project.capture();
        project.create();
        let mut records = project.0.read_repeaters().unwrap();
        records[0].response = Some(base64::encode("stale"));
        project.0.write_repeaters(&records).unwrap();
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap().to_string();
        let task = tokio::spawn(async move {
            let (_socket, _) = listener.accept().await.unwrap();
            std::future::pending::<()>().await;
        });
        let error = project
            .0
            .execute_repeater(input(
                json!({"id":1,"changes":{"address":address,"name":"timed out"}}),
            ))
            .await
            .err()
            .unwrap();
        task.abort();
        assert!(error.message.contains("timed out"));
        let saved = project.0.get_repeater(input(json!({"id":1}))).unwrap().0;
        assert_eq!(saved["name"], "timed out");
        assert!(saved["response"].is_null());
    }

    #[tokio::test]
    async fn output_limit_error_after_send_does_not_discard_response() {
        let mut project = Project::new(true);
        project.capture();
        project.create();
        project.0.settings.max_result_bytes = 1024;
        let (address, received) = local_server(vec![b'x'; 5000]).await;
        let error = project
            .0
            .execute_repeater(input(json!({"id":1,"changes":{"address":address}})))
            .await
            .err()
            .unwrap();
        assert!(error.message.contains("response saved"));
        received.await.unwrap();
        assert!(project.0.read_repeaters().unwrap()[0].response.is_some());
    }

    struct Project(CrusterMcpServer);

    impl Project {
        fn new(enabled: bool) -> Self {
            let path = std::env::temp_dir()
                .join(format!("cruster-mcp-test-{:016x}", rand::random::<u64>()));
            fs::create_dir(&path).unwrap();
            Self(CrusterMcpServer::new(
                path,
                AuditConfig::default(),
                McpConfig {
                    allow_dangerous_tools: enabled,
                    max_result_bytes: 1024 * 1024,
                    request_timeout_seconds: 1,
                },
            ))
        }

        fn capture(&self) {
            let request = parse_raw("POST /submit?foo=bar HTTP/1.1\r\nHost: example.test\r\nX-Repeat: one\r\nX-Repeat: two\r\nContent-Length: 7\r\n\r\nfoo=bar", "http://127.0.0.1:8080/").unwrap();
            let pair = crate::http_storage::serializable::SerializableProxyData {
                index: 42,
                request: SerializableHTTPRequest::from(&request),
                response: None,
            };
            fs::write(
                self.0.history_path(),
                format!("{}\n", serde_json::to_string(&pair).unwrap()),
            )
            .unwrap();
        }

        fn create(&self) -> serde_json::Value {
            self.0
                .create_repeater(input(
                    json!({"history_id": 42, "changes": {"name": "agent test"}}),
                ))
                .unwrap()
                .0
        }
    }

    impl Drop for Project {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(self.0.project.as_ref());
        }
    }

    #[test]
    fn raw_roundtrip_preserves_body_and_duplicate_headers() {
        for separator in ["\n", "\r\n"] {
            let body = "first\r\n\r\nsecond\n\nlast  \n";
            let raw = format!("POST /p?foo=bar HTTP/1.1{separator}Host: virtual.test{separator}X-Dup:1{separator}X-Dup: 2{separator}{separator}{body}");
            let request = parse_raw(&raw, "http://127.0.0.1:8080/old").unwrap();
            assert_eq!(request.body, body.as_bytes());
            assert_eq!(request.headers.get_all("x-dup").iter().count(), 2);
            assert_eq!(request.uri, "http://127.0.0.1:8080/p?foo=bar");
            let restored = parse_raw(&render_raw(&request).unwrap(), &request.uri).unwrap();
            assert_eq!(restored.body, request.body);
            assert_eq!(restored.headers, request.headers);
        }
    }

    #[test]
    fn structured_edits_support_binary_and_header_operations() {
        let mut request = parse_raw(
            "GET /?foo=bar HTTP/1.1\nHost: virtual.test\nX-Remove: yes\nContent-Length: 0\n\n",
            "http://127.0.0.1/",
        )
        .unwrap();
        input::<RequestEdit>(json!({"method": "PATCH", "url": "https://example.test:8443/a?old=1", "path": "/new", "query": "foo=bad", "version": "HTTP/2", "remove_headers": ["x-remove"], "set_headers": [{"name": "X-Dup", "value": "one"}, {"name": "x-dup", "value": "two"}], "body_base64": "AP+A"})).apply(&mut request).unwrap();
        assert_eq!(request.uri, "https://example.test:8443/new?foo=bad");
        assert_eq!(request.method, "PATCH");
        assert_eq!(request.version, "HTTP/2");
        assert_eq!(request.body, vec![0, 255, 128]);
        assert_eq!(request.headers["host"], "virtual.test");
        assert!(!request.headers.contains_key("x-remove"));
        assert!(!request.headers.contains_key("content-length"));
        assert_eq!(request.headers.get_all("x-dup").iter().count(), 2);
        input::<RequestEdit>(json!({"query": "", "headers": [], "body": ""}))
            .apply(&mut request)
            .unwrap();
        assert_eq!(request.uri, "https://example.test:8443/new");
        assert!(request.body.is_empty());
        assert!(request.headers.is_empty());
    }

    #[test]
    fn invalid_edits_are_rejected() {
        let original = parse_raw("GET / HTTP/1.1\n\n", "http://localhost/").unwrap();
        for value in [
            json!({"body": "a", "body_base64": "YQ=="}),
            json!({"body_base64": "!"}),
            json!({"url": "file:///etc/passwd"}),
            json!({"url": "http://user:secret@localhost/"}),
            json!({"version": "HTTP/3"}),
            json!({"method": "bad method"}),
            json!({"headers": [{"name": "X-Test", "value": "bad\r\ninjection"}]}),
            json!({"raw_request": "invalid"}),
        ] {
            assert!(input::<RequestEdit>(value)
                .apply(&mut original.clone())
                .is_err());
        }
        assert!(serde_json::from_value::<RequestEdit>(json!({"typo_body": "test"})).is_err());
    }

    #[test]
    fn saved_repeater_crud_is_compatible_with_cli_and_keeps_history_unchanged() {
        let project = Project::new(true);
        project.capture();
        let history = fs::read(project.0.history_path()).unwrap();
        assert_eq!(project.create()["id"], 1);
        let body = "foo=bad\n\nlast \r\n";
        let edited = project.0.update_repeater(input(json!({"id": 1, "changes": {"name": "edited", "address": "127.0.0.1:9999", "https": true, "redirects": false, "max_redirects": 0, "edits": {"body": body, "query": "foo=bad"}}}))).unwrap().0;
        assert_eq!(edited["parameters"]["address"], "127.0.0.1:9999");
        assert_eq!(edited["parameters"]["https"], true);
        let persisted = fs::read_to_string(project.0.project.join("repeater.jsonl")).unwrap();
        let serial: crate::siv_ui::repeater::RepeaterStateSerializable =
            serde_json::from_str(persisted.trim()).unwrap();
        let state = crate::siv_ui::repeater::RepeaterState::try_from(serial).unwrap();
        let request = state.make_reqwest().unwrap();
        assert_eq!(request.body().unwrap().as_bytes().unwrap(), body.as_bytes());
        assert_eq!(request.headers().get_all("x-repeat").iter().count(), 2);
        assert_eq!(
            request.url().as_str(),
            "https://127.0.0.1:9999/submit?foo=bad"
        );
        assert_eq!(request.headers()["host"], "example.test");
        assert_eq!(
            project
                .0
                .list_repeaters(input(json!({"limit": 1})))
                .unwrap()
                .0["items"][0]["name"],
            "edited"
        );
        assert!(project.0.get_repeater(input(json!({"id": 0}))).is_err());
        project.0.delete_repeater(1).unwrap();
        assert!(project.0.read_repeaters().unwrap().is_empty());
        assert_eq!(history, fs::read(project.0.history_path()).unwrap());
    }

    #[tokio::test]
    async fn passive_mode_denies_writes_and_sends() {
        let project = Project::new(false);
        project.capture();
        assert!(project
            .0
            .create_repeater(input(json!({"history_id": 42})))
            .is_err());
        assert!(project
            .0
            .update_repeater(input(json!({"id": 1, "changes": {}})))
            .is_err());
        assert!(project.0.delete_repeater(1).is_err());
        assert!(project
            .0
            .execute_repeater(input(json!({"id": 1})))
            .await
            .is_err());
        assert!(project
            .0
            .http_request_send(Parameters(input(
                json!({"id": 42, "edits": {"body": "foo=bad"}})
            )))
            .await
            .is_err());
        assert!(!project.0.project.join("repeater.jsonl").exists());
        assert!(project.0.list_repeaters(input(json!({}))).is_ok());
    }

    async fn local_server(response_body: Vec<u8>) -> (String, tokio::task::JoinHandle<Vec<u8>>) {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap().to_string();
        let task = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            let mut request = Vec::new();
            let mut buf = [0u8; 4096];
            loop {
                let count = socket.read(&mut buf).await.unwrap();
                if count == 0 {
                    break;
                }
                request.extend_from_slice(&buf[..count]);
                if let Some(pos) = request.windows(4).position(|w| w == b"\r\n\r\n") {
                    let head = String::from_utf8_lossy(&request[..pos]).to_ascii_lowercase();
                    let length: usize = head
                        .lines()
                        .find_map(|l| l.strip_prefix("content-length: "))
                        .unwrap_or("0")
                        .parse()
                        .unwrap();
                    if request.len() >= pos + 4 + length {
                        break;
                    }
                }
            }
            socket
                .write_all(
                    format!(
                        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                        response_body.len()
                    )
                    .as_bytes(),
                )
                .await
                .unwrap();
            socket.write_all(&response_body).await.unwrap();
            request
        });
        (address, task)
    }

    #[tokio::test]
    async fn history_send_edits_the_actual_wire_request_without_changing_capture() {
        let project = Project::new(true);
        project.capture();
        let history = fs::read(project.0.history_path()).unwrap();
        let (address, received) = local_server(b"done".to_vec()).await;
        let result = project.0.http_request_send(Parameters(input(json!({"id": 42, "edits": {"url": format!("http://{}/submit?foo=bad", address), "body": "foo=bad\n\nend\n"}})))).await.unwrap().0;
        let wire = String::from_utf8(received.await.unwrap()).unwrap();
        assert!(wire.starts_with("POST /submit?foo=bad HTTP/1.1\r\n"));
        assert!(wire.ends_with("foo=bad\n\nend\n"));
        assert!(wire.contains("host: example.test\r\n"));
        assert_eq!(
            result["structured_response"]["body"],
            base64::encode("done")
        );
        assert_eq!(history, fs::read(project.0.history_path()).unwrap());
    }

    #[tokio::test]
    async fn repeater_send_saves_large_response_and_reads_it_in_bounded_chunks() {
        let mut project = Project::new(true);
        project.capture();
        project.create();
        project.0.settings.max_result_bytes = 1024;
        let (address, received) = local_server(vec![b'x'; 5000]).await;
        let result = project.0.execute_repeater(input(json!({"id": 1, "include_bodies": false, "changes": {"address": address, "edits": {"body": "foo=bad"}}}))).await.unwrap().0;
        assert_eq!(result["repeater_id"], 1);
        assert!(result["response"].is_null());
        assert!(received.await.unwrap().ends_with(b"foo=bad"));
        assert!(project.0.get_repeater(input(json!({"id": 1}))).is_err());
        let mut output = Vec::new();
        let mut offset = 0;
        loop {
            let chunk = project
                .0
                .get_repeater_content(input(
                    json!({"id": 1, "part": "response", "offset": offset, "limit": 65536}),
                ))
                .unwrap()
                .0;
            assert!(serde_json::to_vec(&chunk).unwrap().len() <= 1024);
            output.extend(base64::decode(chunk["data_base64"].as_str().unwrap()).unwrap());
            if chunk["next_offset"].is_null() {
                break;
            }
            offset = chunk["next_offset"].as_u64().unwrap() as usize;
        }
        assert!(output.ends_with(&vec![b'x'; 5000]));
        let serial: crate::siv_ui::repeater::RepeaterStateSerializable = serde_json::from_str(
            fs::read_to_string(project.0.project.join("repeater.jsonl"))
                .unwrap()
                .trim(),
        )
        .unwrap();
        assert!(crate::siv_ui::repeater::RepeaterState::try_from(serial).is_ok());
    }

    #[test]
    fn authored_raw_request_and_url_override_work() {
        let project = Project::new(true);
        let result = project.0.create_repeater(input(json!({"raw_request": "PUT /new HTTP/1.1\nHost: custom.test\n\nbody", "changes": {"address": "127.0.0.1:8080", "https": false}}))).unwrap().0;
        assert_eq!(result["id"], 1);
        let result = project.0.update_repeater(input(json!({"id": 1, "changes": {"edits": {"url": "https://[::1]:8443/changed?q=1", "raw_request": "POST /raw HTTP/2\nHost: independent.test\n\nnew"}}}))).unwrap().0;
        assert_eq!(result["parameters"]["address"], "[::1]:8443");
        assert_eq!(result["parameters"]["https"], true);
        assert!(result["request"]
            .as_str()
            .unwrap()
            .starts_with("POST /changed?q=1 HTTP/2"));
    }
}
