//! HTTP CLI ranges, filters and formats with bounded pagination and follow cursors.
use std::{
    fs::File,
    io::{BufRead, BufReader, Seek, SeekFrom},
    path::Path,
    time::{Duration, Instant},
};

use rmcp::{handler::server::wrapper::Json, schemars, ErrorData as McpError};
use serde::Deserialize;
use serde_json::{json, Value};

use super::{internal_error, CrusterMcpServer};
use crate::cli::http::{read_history_line, show};
use crate::http_storage::{serializable::SerializableProxyData, RequestResponsePair};

fn invalid(e: impl ToString) -> McpError {
    McpError::invalid_params(e.to_string(), None)
}
fn cli_invalid(e: crate::cli::CrusterCLIError) -> McpError {
    invalid(Into::<String>::into(e))
}
fn yes() -> bool {
    true
}

#[derive(Debug, Clone, Copy, Default, Deserialize, schemars::JsonSchema, PartialEq)]
#[serde(rename_all = "snake_case")]
pub(super) enum HistoryFormat {
    #[default]
    Summary,
    Urls,
    Structured,
    Raw,
    Pretty,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(super) struct HistoryQueryInput {
    /// CLI INDEX: n (default 100), a, n-m (1-based inclusive), -n, iN or Ni.
    range: Option<String>,
    /// CLI --index; searches the whole history, overriding the range.
    id: Option<usize>,
    #[serde(default)]
    format: HistoryFormat,
    /// Regex: domain HTTP matcher, except raw mode which searches original JSONL.
    filter: Option<String>,
    /// CLI --extract: method/status/host/path=value, prefix matching.
    extract: Option<String>,
    #[serde(default = "yes")]
    include_bodies: bool,
    #[serde(default)]
    no_body: bool,
    /// Offset in the filtered selection; preserve all other inputs between pages.
    #[serde(default)]
    offset: usize,
    limit: Option<usize>,
    /// Resume an oversized item's Base64 JSON fragment (decoded byte offset).
    #[serde(default)]
    content_offset: usize,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(super) struct HistoryFollowInput {
    /// Byte position returned as next_cursor; never use an HTTP pair ID here.
    cursor: Option<u64>,
    /// On the initial call skip complete old lines, retaining any partial tail.
    #[serde(default)]
    no_old_lines: bool,
    #[serde(default)]
    urls: bool,
    /// Bounded long poll, at most 30000 ms. Default 1000.
    wait_ms: Option<u64>,
    limit: Option<usize>,
    #[serde(default)]
    content_offset: usize,
}

#[derive(Debug, Clone, Copy, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub(super) enum HistoryContentPart {
    Request,
    Response,
    Record,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(super) struct HistoryContentInput {
    id: usize,
    /// Request/response mean exact decoded body bytes; record means stored JSONL bytes.
    part: HistoryContentPart,
    #[serde(default)]
    offset: usize,
    length: Option<usize>,
}

struct Row {
    start: u64,
    end: u64,
    raw: String,
    record: SerializableProxyData,
}
fn complete_end(path: &Path) -> Result<u64, McpError> {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(0),
        Err(e) => return Err(internal_error(e)),
    };
    let len = file.metadata().map_err(internal_error)?.len();
    let mut reader = BufReader::new(std::io::Read::take(file, len));
    let mut end = 0;
    // fill_buf avoids allocating or parsing skipped historical bodies.
    loop {
        let buf = reader.fill_buf().map_err(internal_error)?;
        if buf.is_empty() {
            break;
        }
        let n = buf.len();
        if let Some(last) = buf.iter().rposition(|b| *b == b'\n') {
            end = reader_position(len, &reader) + last as u64 + 1;
        }
        reader.consume(n);
    }
    Ok(end)
}
fn reader_position(len: u64, reader: &BufReader<std::io::Take<File>>) -> u64 {
    len - reader.get_ref().limit() - reader.buffer().len() as u64
}
// Only newline-terminated rows are consumed in follow mode. Seeking directly to
// the cursor prevents append races from changing the meaning of an offset.
fn rows(path: &Path, cursor: u64, complete_only: bool) -> Result<Vec<Row>, McpError> {
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound && cursor == 0 => return Ok(vec![]),
        Err(e) => return Err(internal_error(e)),
    };
    let snapshot_end = file.metadata().map_err(internal_error)?.len();
    if cursor > snapshot_end {
        return Err(invalid("History was truncated; restart without a cursor"));
    }
    if cursor > 0 {
        file.seek(SeekFrom::Start(cursor - 1))
            .map_err(internal_error)?;
        let mut byte = [0];
        std::io::Read::read_exact(&mut file, &mut byte).map_err(internal_error)?;
        if byte[0] != b'\n' {
            return Err(invalid("Cursor must be a completed line boundary"));
        }
    }
    file.seek(SeekFrom::Start(cursor)).map_err(internal_error)?;
    // Do not chase a live writer past this call's snapshot boundary.
    let mut reader = BufReader::new(std::io::Read::take(file, snapshot_end - cursor));
    let mut result = vec![];
    let mut start = cursor;
    loop {
        let Some(line) = read_history_line(&mut reader, &mut Vec::new(), complete_only)
            .map_err(internal_error)? else { break; };
        let end = start + line.byte_len as u64;
        let raw = line.raw;
        let record = serde_json::from_str(&raw).map_err(internal_error)?;
        result.push(Row {
            start,
            end,
            raw,
            record,
        });
        start = end;
    }
    Ok(result)
}

fn headers(out: &mut String, h: &http::HeaderMap) {
    let mut keys: Vec<_> = h.keys().map(|k| k.as_str()).collect();
    keys.sort();
    for key in keys {
        out.push_str(&format!(
            "{}: {}\n",
            key,
            h.get_all(key)
                .iter()
                .map(|v| String::from_utf8_lossy(v.as_bytes()))
                .collect::<Vec<_>>()
                .join("; ")
        ));
    }
}
fn body(out: &mut String, bytes: &[u8], include: bool) {
    if include && !bytes.is_empty() {
        out.push('\n');
        if bytes.contains(&0) {
            out.push_str("--- INCOMPATIBLE SET OF BYTES ---");
        } else {
            out.push_str(&String::from_utf8_lossy(bytes));
        }
        out.push('\n');
    }
}
fn render(row: &Row, format: HistoryFormat, include: bool) -> Result<Value, McpError> {
    if format == HistoryFormat::Raw {
        return Ok(json!(row.raw));
    }
    if format == HistoryFormat::Structured {
        let mut record = row.record.clone();
        if !include {
            record.request.body = None;
            if let Some(r) = &mut record.response {
                r.body = None;
            }
        }
        return serde_json::to_value(record).map_err(internal_error);
    }
    let pair: RequestResponsePair = row.record.clone().try_into().map_err(internal_error)?;
    let request = pair.request.as_ref();
    if format == HistoryFormat::Urls {
        return Ok(json!({"id":pair.index,"url":request.map(|r| r.uri.as_str())}));
    }
    if format == HistoryFormat::Summary {
        return Ok(json!({"id":pair.index,"method":request.map(|r| &r.method),
            "hostname":request.map(|r| r.get_hostname()),"path":request.map(|r| r.get_request_path()),
            "status":pair.response.as_ref().map(|r| r.status.split(' ').next().unwrap_or("")),
            "length":pair.response.as_ref().map(|r| r.body.len())}));
    }
    let mut out = format!(
        "\n HTTP exchange #{} \n── REQUEST ──────────────────────────────────────────────\n",
        pair.index
    );
    if let Some(r) = request {
        let path = r
            .get_request_path_without_query()
            .unwrap_or_else(|_| r.get_request_path());
        out.push_str(&format!(
            "{} {}{} {}\n",
            r.method,
            path,
            r.get_query().unwrap_or_default(),
            r.version
        ));
        headers(&mut out, &r.headers);
        body(&mut out, &r.body, include);
    } else {
        out.push_str("<EMPTY REQUEST>\n");
    }
    out.push_str("\n── RESPONSE ─────────────────────────────────────────────\n");
    if let Some(r) = &pair.response {
        out.push_str(&format!("{} {}\n", r.version, r.status));
        headers(&mut out, &r.headers);
        body(&mut out, &r.body, include);
    } else {
        out.push_str("<EMPTY RESPONSE>\n");
    }
    out.push_str("──────────────────────────────────────────────────────────\n");
    // Never emit terminal control sequences, including sequences from captured data.
    out = out
        .chars()
        .filter(|c| !c.is_control() || *c == '\n' || *c == '\r' || *c == '\t')
        .collect();
    Ok(json!(out))
}

fn size(v: &Value) -> usize {
    serde_json::to_vec(v).expect("JSON value").len()
}

impl CrusterMcpServer {
    pub(super) fn history_query(&self, input: HistoryQueryInput) -> Result<Json<Value>, McpError> {
        let selected = show::parse_range(input.range.as_deref().unwrap_or("100"))
            .map_err(cli_invalid)?;
        let attr = input.extract.as_deref()
            .map(show::ExtractionAttribute::try_from).transpose().map_err(cli_invalid)?;
        if input.format == HistoryFormat::Raw
            && (input.no_body || !input.include_bodies || attr.is_some())
        {
            return Err(invalid("Raw output cannot omit bodies or use extraction"));
        }
        let format = if selected.selects_id(input.id) {
            HistoryFormat::Pretty
        } else {
            input.format
        };
        let settings = show::ShowSettings {
            filter: input.filter,
            attribute: attr,
            index: input.id,
            raw: format == HistoryFormat::Raw,
            ..Default::default()
        };
        let mut values = vec![];
        show::query(&selected, self.history_path(), &settings, |raw, _pair| {
            // MCP rendering/pagination is separate from CLI's stdout presentation.
            let row = Row {
                start: 0,
                end: 0,
                raw: raw.to_owned(),
                record: serde_json::from_str(raw)?,
            };
            let value = render(&row, format, input.include_bodies && !input.no_body)
                .map_err(|e| crate::cli::CrusterCLIError::from(e.to_string()))?;
            values.push(value);
            Ok(())
        }).map_err(cli_invalid)?;
        self.history_page(
            &values,
            input.offset,
            input.content_offset,
            input.limit.unwrap_or(20).clamp(1, 100),
            None,
        )
    }

    // Pages use ordinary JSON items whenever possible. An oversized item is a
    // stream of Base64 chunks of its UTF-8 JSON encoding, not lossy text slices.
    fn history_page(
        &self,
        values: &[Value],
        offset: usize,
        content_offset: usize,
        limit: usize,
        cursors: Option<(&[u64], u64)>,
    ) -> Result<Json<Value>, McpError> {
        if offset > values.len() || (offset == values.len() && content_offset != 0) {
            return Err(invalid("Invalid page offset"));
        }
        let envelope = |items: &Vec<Value>, next: usize, part: usize| {
            if let Some((positions, end)) = cursors {
                json!({"items":items,"next_cursor":positions.get(next).copied().unwrap_or(end),"next_content_offset":part,"has_more":next < values.len()})
            } else {
                json!({"items":items,"next_offset":if next < values.len() {Some(next)} else {None},"next_content_offset":part})
            }
        };
        let mut items = vec![];
        let mut next = offset;
        let mut part = content_offset;
        while next < values.len() && items.len() < limit {
            let v = &values[next];
            if part == 0 {
                items.push(v.clone());
                if size(&envelope(&items, next + 1, 0)) <= self.settings.max_result_bytes {
                    next += 1;
                    continue;
                }
                items.pop();
                if !items.is_empty() {
                    break;
                }
            }
            let bytes = serde_json::to_vec(v).map_err(internal_error)?;
            if part >= bytes.len() {
                return Err(invalid("Invalid content_offset"));
            }
            let mut n =
                (bytes.len() - part).min(self.settings.max_result_bytes.saturating_mul(3) / 4);
            loop {
                if n == 0 {
                    return Err(invalid(
                        "max_result_bytes is too small for a chunk envelope",
                    ));
                }
                let finished = part + n == bytes.len();
                let chunk = json!({"encoding":"base64","media_type":"application/json","offset":part,"total_bytes":bytes.len(),"data":base64::encode(&bytes[part..part+n])});
                items.push(chunk);
                let candidate = envelope(
                    &items,
                    if finished { next + 1 } else { next },
                    if finished { 0 } else { part + n },
                );
                if size(&candidate) <= self.settings.max_result_bytes {
                    return self.bounded_json(&candidate);
                }
                items.pop();
                n /= 2;
            }
        }
        part = 0;
        self.bounded_json(&envelope(&items, next, part))
    }

    pub(super) async fn history_follow(
        &self,
        input: HistoryFollowInput,
    ) -> Result<Json<Value>, McpError> {
        let path = self.history_path();
        let mut cursor = input.cursor.unwrap_or(0);
        if input.cursor.is_none() && input.content_offset != 0 {
            return Err(invalid("content_offset requires cursor"));
        }
        if input.cursor.is_none() && input.no_old_lines {
            cursor = complete_end(&path)?;
        }
        let deadline =
            Instant::now() + Duration::from_millis(input.wait_ms.unwrap_or(1000).min(30000));
        loop {
            let records = rows(&path, cursor, true)?;
            if !records.is_empty() || Instant::now() >= deadline {
                let format = if input.urls {
                    HistoryFormat::Urls
                } else {
                    HistoryFormat::Summary
                };
                let values = records
                    .iter()
                    .map(|r| render(r, format, false))
                    .collect::<Result<Vec<_>, _>>()?;
                let positions: Vec<_> = records.iter().map(|r| r.start).collect();
                let end = records.last().map(|r| r.end).unwrap_or(cursor);
                return self.history_page(
                    &values,
                    0,
                    input.content_offset,
                    input.limit.unwrap_or(20).clamp(1, 100),
                    Some((&positions, end)),
                );
            }
            tokio::time::sleep(
                Duration::from_millis(50).min(deadline.saturating_duration_since(Instant::now())),
            )
            .await;
        }
    }

    pub(super) fn history_content_get(
        &self,
        input: HistoryContentInput,
    ) -> Result<Json<Value>, McpError> {
        let record = rows(&self.history_path(), 0, false)?
            .into_iter()
            .find(|r| r.record.index == input.id)
            .ok_or_else(|| invalid("HTTP pair ID not found"))?;
        let bytes = match input.part {
            HistoryContentPart::Record => record.raw.into_bytes(),
            HistoryContentPart::Request => {
                base64::decode(record.record.request.body.as_deref().unwrap_or(""))
                    .map_err(internal_error)?
            }
            HistoryContentPart::Response => {
                let response = record
                    .record
                    .response
                    .ok_or_else(|| invalid("No response"))?;
                base64::decode(response.body.as_deref().unwrap_or("")).map_err(internal_error)?
            }
        };
        if input.offset > bytes.len() {
            return Err(invalid("Offset exceeds content length"));
        }
        if input.length == Some(0) {
            return Err(invalid("length must be positive"));
        }
        let mut n = input
            .length
            .unwrap_or(65536)
            .min(bytes.len() - input.offset)
            .min(self.settings.max_result_bytes);
        loop {
            let end = input.offset + n;
            let value = json!({"id":input.id,"encoding":"base64","offset":input.offset,"total_bytes":bytes.len(),"data":base64::encode(&bytes[input.offset..end]),"next_offset":if end < bytes.len() {Some(end)} else {None}});
            if size(&value) <= self.settings.max_result_bytes {
                return self.bounded_json(&value);
            }
            if n <= 1 {
                return Err(invalid(
                    "max_result_bytes is too small for a chunk envelope",
                ));
            }
            n /= 2;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{AuditConfig, McpConfig};
    use std::{
        fs::{self, OpenOptions},
        io::Write,
    };

    struct Fixture {
        server: CrusterMcpServer,
        path: std::path::PathBuf,
    }
    impl Fixture {
        fn new(budget: usize) -> Self {
            let path = std::env::temp_dir().join(format!(
                "cruster-history-{}-{}",
                std::process::id(),
                rand::random::<u64>()
            ));
            fs::create_dir(&path).unwrap();
            let server = CrusterMcpServer::new(
                path.clone(),
                AuditConfig::default(),
                McpConfig {
                    allow_dangerous_tools: false,
                    max_result_bytes: budget,
                    request_timeout_seconds: 1,
                },
            );
            Self { server, path }
        }
        fn write(&self, records: &[Value]) {
            let mut file = File::create(self.server.history_path()).unwrap();
            for record in records {
                writeln!(file, "{}", record).unwrap();
            }
        }
        fn query(&self, input: Value) -> Result<Value, McpError> {
            self.server
                .history_query(serde_json::from_value(input).unwrap())
                .map(|v| v.0)
        }
        fn content(&self, input: Value) -> Value {
            self.server
                .history_content_get(serde_json::from_value(input).unwrap())
                .unwrap()
                .0
        }
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }
    fn record(id: usize) -> Value {
        json!({"index":id,"request":{"method":"GET","scheme":"https://","host":"uri.example:443","path":"/api","query":"?x=1","version":"HTTP/1.1",
            "headers":[{"key":"host","encoding":"utf-8","value":"Header.Example:8443"},{"key":"x-test","encoding":"utf-8","value":"marker-header"}],"body":base64::encode(b"request secret")},
            "response":{"status":"201 Created","version":"HTTP/1.1","headers":[],"body":base64::encode(b"response secret")}})
    }
    fn ids(value: &Value) -> Vec<usize> {
        value["items"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v["id"].as_u64().unwrap() as usize)
            .collect()
    }

    #[test]
    fn mcp_uses_cli_query_selection_and_filters_before_pagination() {
        let f = Fixture::new(100000);
        let mut post = record(3);
        post["request"]["method"] = json!("POST");
        f.write(&[record(40), post, record(91), record(12)]);
        for range in ["a", "3", "2-4", "-2", "i91", "91i"] {
            let selected = show::parse_range(range).unwrap();
            let settings = show::ShowSettings {
                attribute: Some(show::ExtractionAttribute::try_from("method=gE").unwrap()),
                filter: Some("response secret".to_owned()),
                ..Default::default()
            };
            let mut expected = Vec::new();
            show::query(&selected, f.server.history_path(), &settings, |_, pair| {
                expected.push(pair.unwrap().index);
                Ok(())
            }).unwrap();
            let page = f.query(json!({"range":range,"extract":"method=gE","filter":"response secret"})).unwrap();
            if selected.selects_id(None) {
                assert!(page["items"][0].as_str().unwrap().contains(&format!("exchange #{}", expected[0])));
            } else {
                assert_eq!(ids(&page), expected);
                let page = f.query(json!({"range":range,"extract":"method=gE","filter":"response secret","offset":1,"limit":1})).unwrap();
                assert_eq!(ids(&page), expected.into_iter().skip(1).take(1).collect::<Vec<_>>());
            }
        }
    }

    #[test]
    fn shared_query_preserves_cli_raw_and_incremental_error_behavior() {
        let f = Fixture::new(100000);
        let path = f.server.history_path();
        fs::write(&path, format!("{}\r\nnot-json\n", record(7))).unwrap();
        let range = show::parse_range("a").unwrap();
        let mut delivered = Vec::new();
        let result = show::query(&range, &path, &show::ShowSettings::default(), |_, pair| {
            delivered.push(pair.unwrap().index);
            Ok(())
        });
        assert!(result.is_err());
        assert_eq!(delivered, vec![7]); // CLI must not buffer away earlier stdout.
        let settings = show::ShowSettings {
            raw: true,
            filter: Some("not-json".to_owned()),
            ..Default::default()
        };
        let mut raw = Vec::new();
        show::query(&range, &path, &settings, |line, pair| {
            assert!(pair.is_none());
            raw.push(line.to_owned());
            Ok(())
        }).unwrap();
        assert_eq!(raw, vec!["not-json"]);
        let settings = show::ShowSettings { raw: true, index: Some(7), ..Default::default() };
        show::query(&show::parse_range("0").unwrap(), &path, &settings, |_, pair| {
            assert_eq!(pair.unwrap().index, 7); // ID overrides raw and positional range.
            Ok(())
        }).unwrap();
        // A malformed row outside the selected CLI range is not decoded by MCP.
        assert_eq!(ids(&f.query(json!({"range":"1"})).unwrap()), vec![7]);
        assert!(f.query(json!({"range":"a"})).is_err());
    }

    #[test]
    fn legacy_history_list_get_keep_structured_contracts() {
        use rmcp::handler::server::wrapper::Parameters;
        let f = Fixture::new(100000);
        f.write(&[record(40), record(3), record(91)]);
        let list = f.server.http_history_list(Parameters(
            serde_json::from_value(json!({"offset":1,"limit":1,"method":"get","host":"uri.example","status":"20"})).unwrap()
        )).unwrap().0;
        assert_eq!(list["items"][0]["index"], 3);
        assert!(list["items"][0]["request"]["body"].is_null());
        assert_eq!(list["next_offset"], 2);
        let item = f.server.http_history_get(Parameters(
            serde_json::from_value(json!({"id":91,"include_bodies":true})).unwrap()
        )).unwrap().0;
        assert_eq!(item, record(91)); // Legacy ID lookup remains structured, not pretty.
        assert!(f.server.http_history_get(Parameters(
            serde_json::from_value(json!({"id":999})).unwrap()
        )).is_err());
    }

    #[test]
    fn schemas_defaults_limits_and_empty_history() {
        for schema in [
            schemars::schema_for!(HistoryQueryInput),
            schemars::schema_for!(HistoryFollowInput),
            schemars::schema_for!(HistoryContentInput),
        ] {
            let value = serde_json::to_value(schema).unwrap();
            assert_eq!(value["additionalProperties"], false);
        }
        assert!(
            serde_json::from_value::<HistoryQueryInput>(json!({"path":"/etc/passwd"})).is_err()
        );
        let f = Fixture::new(100000);
        assert!(f.query(json!({})).is_err());
        f.write(&(0..105).map(record).collect::<Vec<_>>());
        let p = f.query(json!({"offset":99,"limit":1000})).unwrap();
        assert_eq!(ids(&p), vec![99]); // default range is first 100, before filtering/paging
        let p = f.query(json!({"range":"a","limit":1000})).unwrap();
        assert_eq!(p["items"].as_array().unwrap().len(), 100);
        assert_eq!(p["next_offset"], 100);
        let mut single = Fixture::new(100000);
        single.write(&[record(1)]);
        let exact = size(&single.query(json!({})).unwrap());
        single.server.settings.max_result_bytes = exact;
        assert_eq!(size(&single.query(json!({})).unwrap()), exact);
        single.server.settings.max_result_bytes = exact - 1;
        let smaller = single.query(json!({}));
        if let Ok(page) = smaller {
            assert!(size(&page) < exact);
        }
    }

    #[tokio::test]
    async fn no_old_lines_skips_unparsed_history_and_missing_file() {
        let f = Fixture::new(1000);
        let p = f
            .server
            .history_follow(serde_json::from_value(json!({"wait_ms":0})).unwrap())
            .await
            .unwrap()
            .0;
        assert_eq!(p["next_cursor"], 0);
        fs::write(f.server.history_path(), b"not json\npartial").unwrap();
        let p = f
            .server
            .history_follow(
                serde_json::from_value(json!({"no_old_lines":true,"wait_ms":0})).unwrap(),
            )
            .await
            .unwrap()
            .0;
        assert_eq!(p["next_cursor"], 9);
        assert!(p["items"].as_array().unwrap().is_empty());
        // Verify the boundary scan across several internal reader buffers.
        let old = format!("{}\n{}", "x".repeat(20000), "z".repeat(15000));
        fs::write(f.server.history_path(), old).unwrap();
        assert_eq!(complete_end(&f.server.history_path()).unwrap(), 20001);
    }

    #[test]
    fn cli_ranges_and_independent_ids() {
        let f = Fixture::new(100000);
        f.write(&[record(40), record(3), record(91), record(12)]);
        for (range, expected) in [
            ("2", vec![40, 3]),
            ("a", vec![40, 3, 91, 12]),
            ("2-3", vec![3, 91]),
            ("-2", vec![91, 12]),
            ("-99", vec![40, 3, 91, 12]),
        ] {
            assert_eq!(ids(&f.query(json!({"range":range})).unwrap()), expected);
        }
        for range in ["i91", "91i"] {
            let page = f.query(json!({"range":range,"format":"urls"})).unwrap();
            assert!(page["items"][0].as_str().unwrap().contains("exchange #91"));
        }
        let page = f
            .query(json!({"range":"1","id":12,"format":"raw"}))
            .unwrap();
        assert!(page["items"][0].as_str().unwrap().contains("exchange #12"));
        for range in ["0", "-0", "8-10", "0-1", "3-2", "i999", "garbage", "1-2-3"] {
            assert!(f.query(json!({"range":range})).is_err(), "{range}");
        }
    }

    #[test]
    fn domain_filters_and_exact_cli_extraction() {
        let f = Fixture::new(100000);
        f.write(&[record(7)]);
        for re in [
            "request secret",
            "response secret",
            "marker-header",
            "GET https://",
            "201 Created",
        ] {
            assert_eq!(ids(&f.query(json!({"filter":re})).unwrap()), vec![7]);
        }
        // HTTP bodies are decoded by the domain matcher, not searched as Base64 JSON.
        assert!(f.query(json!({"filter":"cmVxdWVzdCBzZWNyZXQ="})).is_err());
        for attr in [
            "method=gE",
            "METHOD=get",
            "host=Header.Example:",
            "path=/api?x",
            "status=20",
        ] {
            assert!(f.query(json!({"extract":attr})).is_ok(), "{attr}");
        }
        for attr in [
            "host=header.example",
            "host=uri.example",
            "path=/API",
            "status=404",
            "unknown=x",
            "method=a=b",
            "path=/api?x=",
            "method",
        ] {
            assert!(f.query(json!({"extract":attr})).is_err(), "{attr}");
        }
        assert!(f.query(json!({"filter":"["})).is_err());
        assert!(f
            .query(json!({"format":"raw","filter":"request secret"}))
            .is_err());
        assert!(f
            .query(json!({"format":"raw","filter":"cmVxdWVzdCBzZWNyZXQ="}))
            .is_ok());
        assert!(f
            .query(json!({"filter":"response secret","extract":"method=post"}))
            .is_err());
    }

    #[test]
    fn formats_bodies_and_missing_response() {
        let f = Fixture::new(100000);
        let mut r = record(7);
        r["request"]["body"] = json!(base64::encode([0, 255, 1, 128]));
        f.write(&[r.clone()]);
        let structured = f.query(json!({"format":"structured"})).unwrap();
        assert_eq!(structured["items"][0], r);
        for flags in [
            json!({"format":"structured","no_body":true}),
            json!({"format":"structured","include_bodies":false}),
        ] {
            let p = f.query(flags).unwrap();
            assert!(p["items"][0]["request"]["body"].is_null());
            assert!(p["items"][0]["response"]["body"].is_null());
        }
        let raw = f.query(json!({"format":"raw"})).unwrap();
        assert_eq!(
            serde_json::from_str::<Value>(raw["items"][0].as_str().unwrap()).unwrap(),
            r
        );
        assert_eq!(
            f.query(json!({"format":"urls"})).unwrap()["items"][0]["url"],
            "https://uri.example:443/api?x=1"
        );
        let p = f.query(json!({"format":"pretty"})).unwrap();
        let text = p["items"][0].as_str().unwrap();
        assert!(text.contains("GET /api?x=1 HTTP/1.1"));
        assert!(text.contains("INCOMPATIBLE SET OF BYTES"));
        assert!(text.contains("HTTP/1.1 201 Created"));
        assert!(!text.contains('\x1b'));
        let p = f.query(json!({"format":"pretty","no_body":true})).unwrap();
        assert!(!p["items"][0].as_str().unwrap().contains("response secret"));
        r["response"] = Value::Null;
        f.write(&[r]);
        assert!(f.query(json!({"format":"pretty"})).unwrap()["items"][0]
            .as_str()
            .unwrap()
            .contains("<EMPTY RESPONSE>"));
        assert!(f.query(json!({"extract":"status=2"})).is_err());
        for input in [
            json!({"format":"raw","no_body":true}),
            json!({"format":"raw","extract":"method=get"}),
            json!({"format":"raw","include_bodies":false}),
        ] {
            assert!(f.query(input).is_err());
        }
    }

    #[test]
    fn bounded_pages_and_oversized_item_roundtrip() {
        let f = Fixture::new(400);
        let mut r = record(7);
        r["request"]["body"] = json!(base64::encode(vec![255; 8000]));
        f.write(&[r.clone(), record(8), record(9)]);
        let mut offset = 0;
        let mut content_offset = 0;
        let mut reconstructed = vec![];
        let mut count = 0;
        loop {
            let p = f.query(json!({"range":"a","format":"structured","offset":offset,"content_offset":content_offset})).unwrap();
            assert!(size(&p) <= 400);
            for item in p["items"].as_array().unwrap() {
                if item["encoding"] == "base64" {
                    reconstructed.extend(base64::decode(item["data"].as_str().unwrap()).unwrap());
                    if reconstructed.len() == item["total_bytes"].as_u64().unwrap() as usize {
                        let value: Value = serde_json::from_slice(&reconstructed).unwrap();
                        if count == 0 {
                            assert_eq!(value, r);
                        }
                        reconstructed.clear();
                        count += 1;
                    }
                } else {
                    count += 1;
                }
            }
            if p["next_offset"].is_null() {
                break;
            }
            let next = p["next_offset"].as_u64().unwrap();
            let part = p["next_content_offset"].as_u64().unwrap();
            assert!(next > offset || part > content_offset);
            offset = next;
            content_offset = part;
        }
        assert_eq!(count, 3);
        assert!(f.query(json!({"offset":4})).is_err());
        assert!(f.query(json!({"content_offset":999999})).is_err());
        let tiny = Fixture::new(8);
        tiny.write(&[record(1)]);
        assert!(tiny.query(json!({})).is_err());
    }

    #[test]
    fn exact_binary_and_record_chunks() {
        let f = Fixture::new(300);
        let mut r = record(7);
        let bytes: Vec<u8> = (0..10000).map(|n| (n % 256) as u8).collect();
        r["request"]["body"] = json!(base64::encode(&bytes));
        r["response"]["body"] = json!(base64::encode(&bytes));
        f.write(&[r.clone()]);
        for part in ["request", "response", "record"] {
            let mut offset = 0;
            let mut result = vec![];
            loop {
                let p = f.content(json!({"id":7,"part":part,"offset":offset,"length":100000}));
                assert!(size(&p) <= 300);
                result.extend(base64::decode(p["data"].as_str().unwrap()).unwrap());
                if p["next_offset"].is_null() {
                    break;
                }
                let next = p["next_offset"].as_u64().unwrap();
                assert!(next > offset);
                offset = next;
            }
            if part == "record" {
                assert_eq!(serde_json::from_slice::<Value>(&result).unwrap(), r);
            } else {
                assert_eq!(result, bytes);
            }
        }
        let p = f.content(json!({"id":7,"part":"request","offset":bytes.len()}));
        assert_eq!(p["data"], "");
        assert!(p["next_offset"].is_null());
        assert!(f
            .server
            .history_content_get(
                serde_json::from_value(json!({"id":7,"part":"request","offset":10001})).unwrap()
            )
            .is_err());
    }

    #[tokio::test]
    async fn follow_live_append_partial_and_no_loss() {
        let f = Fixture::new(500);
        f.write(&[record(1), record(2), record(3)]);
        let initial = f
            .server
            .history_follow(
                serde_json::from_value(json!({"no_old_lines":true,"wait_ms":0})).unwrap(),
            )
            .await
            .unwrap()
            .0;
        assert!(initial["items"].as_array().unwrap().is_empty());
        let cursor = initial["next_cursor"].as_u64().unwrap();
        let mut writer = OpenOptions::new()
            .append(true)
            .open(f.server.history_path())
            .unwrap();
        let line = serde_json::to_vec(&record(4)).unwrap();
        writer.write_all(&line[..line.len() / 2]).unwrap();
        writer.flush().unwrap();
        let partial = f
            .server
            .history_follow(serde_json::from_value(json!({"cursor":cursor,"wait_ms":0})).unwrap())
            .await
            .unwrap()
            .0;
        assert_eq!(partial["next_cursor"], cursor);
        let tail = line[line.len() / 2..].to_vec();
        let task = tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(60)).await;
            writer.write_all(&tail).unwrap();
            writer.write_all(b"\n").unwrap();
            writeln!(writer, "{}", record(5)).unwrap();
            writer.flush().unwrap();
        });
        let p = f
            .server
            .history_follow(
                serde_json::from_value(json!({"cursor":cursor,"wait_ms":1000,"limit":1})).unwrap(),
            )
            .await
            .unwrap()
            .0;
        assert_eq!(ids(&p), vec![4]);
        assert!(size(&p) <= 500);
        task.await.unwrap();
        // The next cursor must not jump to the latest file size and lose #5.
        let p = f
            .server
            .history_follow(
                serde_json::from_value(json!({"cursor":p["next_cursor"],"wait_ms":0,"urls":true}))
                    .unwrap(),
            )
            .await
            .unwrap()
            .0;
        assert_eq!(ids(&p), vec![5]);
        assert!(p["items"][0]["url"].is_string());
        assert!(f
            .server
            .history_follow(serde_json::from_value(json!({"cursor":1,"wait_ms":0})).unwrap())
            .await
            .is_err());
        fs::write(f.server.history_path(), b"").unwrap();
        assert!(f
            .server
            .history_follow(serde_json::from_value(json!({"cursor":cursor,"wait_ms":0})).unwrap())
            .await
            .is_err());
    }

    #[tokio::test]
    async fn follow_oversized_url_and_partial_tail_on_start() {
        let f = Fixture::new(300);
        let mut r = record(1);
        r["request"]["path"] = json!(format!("/{}", "x".repeat(5000)));
        f.write(&[r]);
        let mut cursor = 0;
        let mut content_offset = 0;
        let mut bytes = vec![];
        loop {
            let p = f.server.history_follow(serde_json::from_value(json!({"cursor":cursor,"content_offset":content_offset,"urls":true,"wait_ms":0})).unwrap()).await.unwrap().0;
            assert!(size(&p) <= 300);
            bytes.extend(base64::decode(p["items"][0]["data"].as_str().unwrap()).unwrap());
            let next = p["next_cursor"].as_u64().unwrap();
            if next != cursor {
                break;
            }
            let part = p["next_content_offset"].as_u64().unwrap();
            assert!(part > content_offset);
            content_offset = part;
            cursor = next;
        }
        let value: Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(value["id"], 1);
        let mut file = OpenOptions::new()
            .append(true)
            .open(f.server.history_path())
            .unwrap();
        let line = serde_json::to_string(&record(2)).unwrap();
        file.write_all(line.as_bytes()).unwrap();
        file.flush().unwrap();
        let p = f
            .server
            .history_follow(
                serde_json::from_value(json!({"no_old_lines":true,"wait_ms":0})).unwrap(),
            )
            .await
            .unwrap()
            .0;
        writeln!(file).unwrap();
        file.flush().unwrap();
        let p = f
            .server
            .history_follow(
                serde_json::from_value(json!({"cursor":p["next_cursor"],"wait_ms":0})).unwrap(),
            )
            .await
            .unwrap()
            .0;
        assert_eq!(ids(&p), vec![2]);
    }
}
