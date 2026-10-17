use std::{
    collections::HashMap,
    fs::{self, File},
    io::{BufReader, Read},
    path::PathBuf,
    sync::{Arc, Mutex},
};

use base64;
use http::{header::HeaderName, HeaderMap, HeaderValue};
use rustpython::InterpreterConfig;
use rustpython_vm::{
    builtins::{PyCode, PyStrRef},
    compiler::Mode,
    convert::TryFromObject,
    scope::Scope,
    PyRef, Settings, VirtualMachine,
};
use serde::{Deserialize, Serialize};

use crate::{
    cruster_proxy::request_response::{HyperRequestWrapper, HyperResponseWrapper},
    http_sender::send_request_from_wrapper,
    http_storage::RequestResponsePair,
};

use super::{
    types::SerializableSendResultEntry, AuditError, RuleFinalState, RuleResult, RuleSeverity,
};

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum PythonRuleType {
    Active,
    Passive,
}

#[derive(Debug, Clone)]
pub(crate) struct PythonRule {
    path: PathBuf,
    id: String,
    about: String,
    tags: Vec<String>,
    severity: RuleSeverity,
    rule_type: PythonRuleType,
}

#[derive(Debug, Deserialize)]
struct PythonMetadata {
    id: String,
    #[serde(default)]
    about: String,
    #[serde(default)]
    #[allow(dead_code)]
    authors: Vec<String>,
    #[serde(default)]
    #[allow(dead_code)]
    references: Vec<String>,
    #[serde(default)]
    tags: Vec<String>,
    severity: RuleSeverity,
    #[serde(rename = "type")]
    rule_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PythonHeader {
    name: String,
    value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PythonRequest {
    method: String,
    uri: String,
    version: String,
    headers: Vec<PythonHeader>,
    body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PythonResponse {
    status: String,
    version: String,
    headers: Vec<PythonHeader>,
    body: String,
}

#[derive(Debug, Deserialize)]
struct PythonFinding {
    name: String,
    #[serde(default)]
    extracted: Vec<String>,
    #[serde(default)]
    exchanges: Vec<PythonExchange>,
}

#[derive(Debug, Deserialize)]
struct PythonExchange {
    request: PythonRequest,
    response: PythonResponse,
    #[serde(default)]
    payloads: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct PythonRuleLog {
    pub(crate) level: String,
    pub(crate) message: String,
}

impl PythonRule {
    pub(crate) fn from_file(path: &str) -> Result<Self, AuditError> {
        let metadata = read_python_metadata(path)?;
        let rule_type = match metadata.rule_type.as_str() {
            "active" => PythonRuleType::Active,
            "passive" => PythonRuleType::Passive,
            value => {
                return Err(AuditError(format!(
                    "Unknown Python rule type '{}' in '{}'",
                    value, path
                )))
            }
        };

        Ok(Self {
            path: PathBuf::from(path),
            id: metadata.id,
            about: metadata.about,
            tags: metadata.tags,
            severity: metadata.severity,
            rule_type,
        })
    }

    pub(crate) fn get_id(&self) -> &str {
        &self.id
    }
    pub(crate) fn tags(&self) -> &[String] {
        &self.tags
    }
    pub(crate) fn rule_type(&self) -> &PythonRuleType {
        &self.rule_type
    }

    pub(crate) async fn execute(&self, pair: Arc<RequestResponsePair>) -> (RuleFinalState, Vec<PythonRuleLog>) {
        let pair_index = pair.index;
        self.execute_many(vec![pair])
            .await
            .unwrap_or_else(|reason| vec![(
                pair_index,
                RuleFinalState::Failed(reason),
                vec![],
            )])
            .into_iter()
            .next()
            .map(|(_, state, logs)| (state, logs))
            .unwrap_or_else(|| {
                (
                    RuleFinalState::Skipped(format!("Pair {} was not executed", pair_index)),
                    vec![],
                )
            })
    }

    pub(crate) async fn execute_many(
        &self,
        pairs: Vec<Arc<RequestResponsePair>>,
    ) -> Result<Vec<(usize, RuleFinalState, Vec<PythonRuleLog>)>, String> {
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
        let execution = self.execute_many_stream(pairs, tx);
        tokio::pin!(execution);
        let mut results = Vec::new();
        loop {
            tokio::select! {
                result = &mut execution => {
                    result?;
                    while let Ok(item) = rx.try_recv() {
                        results.push(item);
                    }
                    return Ok(results);
                }
                item = rx.recv() => {
                    if let Some(item) = item {
                        results.push(item);
                    }
                }
            }
        }
    }

    pub(crate) async fn execute_many_stream(
        &self,
        pairs: Vec<Arc<RequestResponsePair>>,
        result_tx: tokio::sync::mpsc::UnboundedSender<(usize, RuleFinalState, Vec<PythonRuleLog>)>,
    ) -> Result<(), String> {
        if pairs.is_empty() {
            return Ok(());
        }

        let source = match fs::read_to_string(&self.path) {
            Ok(source) => source,
            Err(err) => {
                let reason = format!("Cannot read Python rule '{}': {}", self.path.display(), err);
                for pair in pairs {
                    let _ = result_tx.send((pair.index, RuleFinalState::Failed(reason.clone()), vec![]));
                }
                return Ok(());
            }
        };
        let path = self.path.to_string_lossy().to_string();
        let rule = self.clone();
        match tokio::task::spawn_blocking(move || rule.run_isolated_batch(&path, &source, pairs, result_tx)).await {
            Ok(()) => Ok(()),
            Err(err) => Err(format!("Python runtime thread crashed: {}", err)),
        }
    }

    fn run_isolated_batch(
        &self,
        path: &str,
        source: &str,
        pairs: Vec<Arc<RequestResponsePair>>,
        result_tx: tokio::sync::mpsc::UnboundedSender<(usize, RuleFinalState, Vec<PythonRuleLog>)>,
    ) {
        let runtime = None;
        let logs = Arc::new(Mutex::new(Vec::<PythonRuleLog>::new()));
        let mut settings = Settings::default();
        // Embedded rule interpreters must not replace the process-wide Ctrl+C
        // handler with Python's KeyboardInterrupt handler. CLI audits should
        // terminate the whole process, not just interrupt one rule worker.
        settings.no_sig_int = true;
        let interpreter = InterpreterConfig::new()
            .settings(settings)
            .init_stdlib()
            .interpreter();

        interpreter.enter(|vm| {
            let bootstrap = match compile_python(vm, BOOTSTRAP, "<cruster>") {
                Ok(code) => code,
                Err(err) => return send_failed_batch(pairs, err, &result_tx),
            };
            let rule = match compile_python(vm, source, path) {
                Ok(code) => code,
                Err(err) => return send_failed_batch(pairs, err, &result_tx),
            };
            let expression = match compile_python_mode(vm, SCAN_EXPRESSION, "<cruster-scan>", Mode::BlockExpr) {
                Ok(code) => code,
                Err(err) => return send_failed_batch(pairs, err, &result_tx),
            };

            for pair in pairs {
                    let index = pair.index;
                    let Some(request) = pair.request.as_ref() else {
                        let _ = result_tx.send((index, RuleFinalState::Skipped(format!("Pair {} has no request", index)), vec![]));
                        continue;
                    };
                    let Some(response) = pair.response.as_ref() else {
                        let _ = result_tx.send((index, RuleFinalState::Skipped(format!("Pair {} has no response", index)), vec![]));
                        continue;
                    };
                    logs.lock().map(|mut entries| entries.clear()).ok();
                    let result = run_isolated_scan(
                        vm,
                        &bootstrap,
                        &rule,
                        &expression,
                        path,
                        PythonRequest::from(request),
                        PythonResponse::from(response),
                        self.rule_type == PythonRuleType::Active,
                        runtime.clone(),
                        logs.clone(),
                    );
                    let pair_logs = logs.lock().map(|entries| entries.clone()).unwrap_or_default();
                    let state = match result {
                        Ok(value) => self.findings_to_state(&pair, &value),
                        Err(err) => RuleFinalState::Failed(err.to_string()),
                    };
                    if result_tx.send((index, state, pair_logs)).is_err() {
                        return;
                    }
            }
        })
    }

    fn findings_to_state(&self, pair: &RequestResponsePair, value: &str) -> RuleFinalState {
        let python_findings: Vec<PythonFinding> = match serde_json::from_str(value) {
            Ok(findings) => findings,
            Err(err) => return RuleFinalState::Failed(format!("Rule '{}' returned invalid findings: {}", self.id, err)),
        };
        if python_findings.is_empty() {
            return RuleFinalState::Finished(None);
        }

        let mut findings = HashMap::with_capacity(python_findings.len());
        for finding in python_findings {
            let exchanges = finding.exchanges.into_iter().map(|exchange| SerializableSendResultEntry {
                request: HyperRequestWrapper::try_from(exchange.request).map(|v| v.to_string()).unwrap_or_default(),
                payloads: exchange.payloads,
                response: HyperResponseWrapper::try_from(exchange.response).map(|v| v.to_string()).unwrap_or_default(),
            }).collect();
            findings.insert(finding.name, (finding.extracted, exchanges));
        }

        RuleFinalState::Finished(Some(RuleResult {
            id: 0,
            rule_id: self.id.clone(),
            about: self.about.clone(),
            protocol: "http".to_string(),
            r#type: match self.rule_type { PythonRuleType::Active => "active", PythonRuleType::Passive => "passive" }.to_string(),
            pair_index: pair.index,
            severity: self.severity.clone(),
            findings,
            initial_request: pair.request.as_ref().unwrap().to_string(),
            initial_response: pair.response.as_ref().unwrap().to_string(),
        }))
    }

}

fn read_python_metadata(path: &str) -> Result<PythonMetadata, AuditError> {
    const PREFIX: &[u8] = b"metadata = ";
    const MAX_METADATA_BYTES: usize = 1024 * 1024;

    let mut reader = BufReader::new(File::open(path)?);
    let mut prefix = [0_u8; PREFIX.len()];
    reader.read_exact(&mut prefix).map_err(|err| {
        AuditError(format!(
            "Python rule '{}' does not start with 'metadata = ': {}",
            path, err
        ))
    })?;
    if prefix != PREFIX {
        return Err(AuditError(format!(
            "Python rule '{}' must start at byte zero with 'metadata = '",
            path
        )));
    }

    let mut object = Vec::new();
    let mut depth = 0_usize;
    let mut in_string = false;
    let mut escaped = false;
    let mut started = false;

    for byte in reader.bytes() {
        let byte = byte?;
        if !started {
            if byte.is_ascii_whitespace() {
                continue;
            }
            if byte != b'{' {
                return Err(AuditError(format!(
                    "Python rule '{}' metadata value must be a JSON object",
                    path
                )));
            }
            started = true;
            depth = 1;
            object.push(byte);
            continue;
        }

        object.push(byte);
        if object.len() > MAX_METADATA_BYTES {
            return Err(AuditError(format!(
                "Python rule '{}' metadata exceeds {} bytes",
                path, MAX_METADATA_BYTES
            )));
        }

        if in_string {
            if escaped {
                escaped = false;
            } else if byte == b'\\' {
                escaped = true;
            } else if byte == b'"' {
                in_string = false;
            }
            continue;
        }

        match byte {
            b'"' => in_string = true,
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    return serde_json::from_slice(&object).map_err(|err| {
                        AuditError(format!(
                            "Invalid leading metadata JSON object in Python rule '{}': {}",
                            path, err
                        ))
                    });
                }
            }
            _ => {}
        }
    }

    Err(AuditError(format!(
        "Python rule '{}' has an incomplete metadata JSON object",
        path
    )))
}

fn compile_python(
    vm: &VirtualMachine,
    source: &str,
    path: &str,
) -> Result<PyRef<PyCode>, AuditError> {
    compile_python_mode(vm, source, path, Mode::Exec)
}

fn compile_python_mode(
    vm: &VirtualMachine,
    source: &str,
    path: &str,
    mode: Mode,
) -> Result<PyRef<PyCode>, AuditError> {
    vm.compile(source, mode, path.to_string())
        .map_err(|err| AuditError(format!("Python compilation failed in '{}': {}", path, err)))
}

fn send_failed_batch(
    pairs: Vec<Arc<RequestResponsePair>>,
    error: AuditError,
    tx: &tokio::sync::mpsc::UnboundedSender<(usize, RuleFinalState, Vec<PythonRuleLog>)>,
) {
    let reason = error.to_string();
    for pair in pairs {
        let _ = tx.send((pair.index, RuleFinalState::Failed(reason.clone()), vec![]));
    }
}

fn run_isolated_scan(
    vm: &VirtualMachine,
    bootstrap: &PyRef<PyCode>,
    rule: &PyRef<PyCode>,
    expression: &PyRef<PyCode>,
    path: &str,
    request: PythonRequest,
    response: PythonResponse,
    allow_send: bool,
    runtime: Option<tokio::runtime::Handle>,
    logs: Arc<Mutex<Vec<PythonRuleLog>>>,
) -> Result<String, AuditError> {
    let scope: Scope = vm.new_scope_with_builtins();
    let native_logs = logs.clone();
    let rule_log = vm.new_function(
        "log",
        move |message: PyStrRef, level: Option<PyStrRef>, vm: &VirtualMachine| {
            let level = level
                .map(|value| value.as_str().to_lowercase())
                .unwrap_or_else(|| "info".to_string());
            if !matches!(level.as_str(), "debug" | "info" | "warning" | "error") {
                return Err(vm.new_value_error(
                    "log level must be one of: debug, info, warning, error".to_string(),
                ));
            }
            native_logs
                .lock()
                .map_err(|_| vm.new_runtime_error("rule log buffer lock poisoned".to_string()))?
                .push(PythonRuleLog { level, message: message.as_str().to_string() });
            Ok(())
        },
    );
    scope.globals.set_item("__cruster_log", rule_log.into(), vm)
        .map_err(|err| python_error(vm, err))?;

    let runtime_for_send = runtime.clone();
    let send = vm.new_function(
        "send",
        move |request_json: PyStrRef, max_redirects: Option<usize>, vm: &VirtualMachine| {
            if !allow_send {
                return Err(vm.new_exception_msg(
                    vm.ctx.exceptions.permission_error.to_owned(),
                    "send() is available only to active rules".to_string(),
                ));
            }
            let request: PythonRequest = serde_json::from_str(request_json.as_str())
                .map_err(|err| vm.new_value_error(format!("invalid request: {}", err)))?;
            let wrapper = HyperRequestWrapper::try_from(request)
                .map_err(|err| vm.new_value_error(err.to_string()))?;
            let redirects = max_redirects.unwrap_or(5);
            let result = if let Some(handle) = runtime_for_send.as_ref() {
                handle.block_on(send_request_from_wrapper(&wrapper, redirects))
            } else {
                tokio::runtime::Runtime::new()
                    .map_err(|err| vm.new_runtime_error(err.to_string()))?
                    .block_on(send_request_from_wrapper(&wrapper, redirects))
            };
            let (response, _) = result.map_err(|err| vm.new_runtime_error(err.to_string()))?;
            serde_json::to_string(&PythonResponse::from(&response))
                .map_err(|err| vm.new_runtime_error(err.to_string()))
        },
    );
    scope.globals.set_item("__cruster_send", send.into(), vm)
        .map_err(|err| python_error(vm, err))?;
    scope.globals.set_item("__file__", vm.new_pyobj(path.to_string()), vm)
        .map_err(|err| python_error(vm, err))?;

    vm.run_code_obj(bootstrap.clone(), scope.clone()).map_err(|err| python_error(vm, err))?;
    vm.run_code_obj(rule.clone(), scope.clone()).map_err(|err| python_error(vm, err))?;
    scope.globals.set_item(
        "__cruster_request_json",
        vm.new_pyobj(serde_json::to_string(&request)?),
        vm,
    ).map_err(|err| python_error(vm, err))?;
    scope.globals.set_item(
        "__cruster_response_json",
        vm.new_pyobj(serde_json::to_string(&response)?),
        vm,
    ).map_err(|err| python_error(vm, err))?;
    let value = vm.run_code_obj(expression.clone(), scope).map_err(|err| python_error(vm, err))?;
    String::try_from_object(vm, value).map_err(|err| python_error(vm, err))
}

fn python_error(
    vm: &VirtualMachine,
    error: rustpython_vm::builtins::PyBaseExceptionRef,
) -> AuditError {
    let mut output = String::new();
    let _ = vm.write_exception(&mut output, &error);
    AuditError(output.trim().to_string())
}

const BOOTSTRAP: &str = r#"
import json as __cruster_json
import sys as __cruster_sys

class HttpRequest:
    def __init__(self, data):
        self.method = data['method']; self.uri = data['uri']; self.version = data['version']
        self.headers = data['headers']; self.body = data['body']
    def clone(self): return HttpRequest(self.to_dict())
    def to_dict(self):
        return {'method': self.method, 'uri': self.uri, 'version': self.version,
                'headers': self.headers, 'body': self.body}

class HttpResponse:
    def __init__(self, data):
        self.status = data['status']; self.version = data['version']
        self.headers = data['headers']; self.body = data['body']
    def to_dict(self):
        return {'status': self.status, 'version': self.version,
                'headers': self.headers, 'body': self.body}

def __cruster_request(raw): return HttpRequest(__cruster_json.loads(raw))
def __cruster_response(raw): return HttpResponse(__cruster_json.loads(raw))
def send(request, max_redirects=5):
    return HttpResponse(__cruster_json.loads(__cruster_send(__cruster_json.dumps(request.to_dict()), max_redirects)))
def finding(name, extracted=None, exchanges=None):
    return {'name': name, 'extracted': extracted or [], 'exchanges': exchanges or []}
def exchange(request, response, payloads=None):
    return {'request': request.to_dict(), 'response': response.to_dict(), 'payloads': payloads or []}
def log(message, level='info'):
    __cruster_log(str(message), level)

# Rules use a normal Python import. The module is supplied by Cruster at runtime;
# its matching type declaration lives in python/cruster/__init__.pyi for editors.
__cruster_module = type(__cruster_json)('cruster')
__cruster_module.HttpRequest = HttpRequest
__cruster_module.HttpResponse = HttpResponse
__cruster_module.send = send
__cruster_module.finding = finding
__cruster_module.exchange = exchange
__cruster_module.log = log
__cruster_sys.modules['cruster'] = __cruster_module
"#;

const SCAN_EXPRESSION: &str = r#"__cruster_json.dumps(
    scan(
        __cruster_request(__cruster_request_json),
        __cruster_response(__cruster_response_json),
    )
)"#;

impl From<&HyperRequestWrapper> for PythonRequest {
    fn from(value: &HyperRequestWrapper) -> Self {
        Self {
            method: value.method.clone(),
            uri: value.uri.clone(),
            version: value.version.clone(),
            headers: headers_to_python(&value.headers),
            body: base64::encode(&value.body),
        }
    }
}

impl TryFrom<PythonRequest> for HyperRequestWrapper {
    type Error = AuditError;
    fn try_from(value: PythonRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            method: value.method,
            uri: value.uri,
            version: value.version,
            headers: headers_from_python(value.headers)?,
            body: base64::decode(value.body)?,
        })
    }
}

impl From<&HyperResponseWrapper> for PythonResponse {
    fn from(value: &HyperResponseWrapper) -> Self {
        Self {
            status: value.status.clone(),
            version: value.version.clone(),
            headers: headers_to_python(&value.headers),
            body: base64::encode(&value.body),
        }
    }
}

impl TryFrom<PythonResponse> for HyperResponseWrapper {
    type Error = AuditError;
    fn try_from(value: PythonResponse) -> Result<Self, Self::Error> {
        Ok(Self {
            status: value.status,
            version: value.version,
            headers: headers_from_python(value.headers)?,
            body: base64::decode(value.body)?,
        })
    }
}

fn headers_to_python(headers: &HeaderMap) -> Vec<PythonHeader> {
    headers
        .iter()
        .map(|(name, value)| PythonHeader {
            name: name.to_string(),
            value: base64::encode(value.as_bytes()),
        })
        .collect()
}

fn headers_from_python(headers: Vec<PythonHeader>) -> Result<HeaderMap, AuditError> {
    let mut result = HeaderMap::new();
    for header in headers {
        result.append(
            HeaderName::from_bytes(header.name.as_bytes())?,
            HeaderValue::from_bytes(&base64::decode(header.value)?)?,
        );
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, time::{SystemTime, UNIX_EPOCH}};

    fn temporary_rule(source: &str) -> PathBuf {
        let suffix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!("cruster-python-rule-{}-{}.py", std::process::id(), suffix));
        fs::write(&path, source).unwrap();
        path
    }

    #[test]
    fn loads_python_metadata() {
        let rule = PythonRule::from_file("rules/example/http/passive/security_headers.py").unwrap();
        assert_eq!(rule.get_id(), "python-missing-security-headers");
        assert_eq!(rule.rule_type(), &PythonRuleType::Passive);
    }

    #[test]
    fn loads_all_bundled_python_metadata() {
        let mut directories = vec![PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("rules")];
        let mut count = 0;
        while let Some(directory) = directories.pop() {
            for entry in fs::read_dir(directory).unwrap() {
                let entry = entry.unwrap();
                let path = entry.path();
                if entry.file_type().unwrap().is_dir() {
                    directories.push(path);
                } else if path.extension().is_some_and(|extension| extension == "py") {
                    PythonRule::from_file(path.to_str().unwrap())
                        .unwrap_or_else(|err| panic!("Invalid bundled rule '{}': {:?}", path.display(), err));
                    count += 1;
                }
            }
        }
        assert!(count > 0, "No bundled Python rules found");
    }

    #[test]
    fn metadata_loading_does_not_execute_python() {
        let path = temporary_rule(
            r#"metadata = {"id":"static-metadata","severity":"info","type":"passive"}
raise RuntimeError("must not run while loading metadata")
"#,
        );
        let rule = PythonRule::from_file(path.to_str().unwrap()).unwrap();
        fs::remove_file(path).unwrap();
        assert_eq!(rule.get_id(), "static-metadata");
    }

    #[test]
    fn rejects_content_before_metadata() {
        let path = temporary_rule(
            "# comment\nmetadata = {\"id\":\"late\",\"severity\":\"info\",\"type\":\"passive\"}\n",
        );
        let result = PythonRule::from_file(path.to_str().unwrap());
        fs::remove_file(path).unwrap();
        assert!(result.is_err());
    }

    #[test]
    fn rejects_non_json_metadata() {
        let path = temporary_rule(
            "metadata = {'id':'python-literal','severity':'info','type':'passive'}\n",
        );
        let result = PythonRule::from_file(path.to_str().unwrap());
        fs::remove_file(path).unwrap();
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn isolated_batch_does_not_preserve_module_globals() {
        let path = temporary_rule(
            r#"metadata = {"id":"isolated-state","severity":"info","type":"passive"}
from cruster import finding
counter = 0
def scan(request, response):
    global counter
    counter += 1
    return [finding("counter", extracted=[str(counter)])]
"#,
        );
        let rule = PythonRule::from_file(path.to_str().unwrap()).unwrap();
        let make_pair = |index| Arc::new(RequestResponsePair {
            request: Some(HyperRequestWrapper {
                uri: format!("http://example.test/{}", index),
                method: "GET".to_string(),
                version: "HTTP/1.1".to_string(),
                headers: HeaderMap::new(),
                body: Vec::new(),
            }),
            response: Some(HyperResponseWrapper {
                status: "200 OK".to_string(),
                version: "HTTP/1.1".to_string(),
                headers: HeaderMap::new(),
                body: Vec::new(),
            }),
            index,
            timestamp: None,
        });

        let results = rule.execute_many(vec![make_pair(1), make_pair(2)]).await.unwrap();
        fs::remove_file(path).unwrap();
        assert_eq!(results.len(), 2);
        for (_, state, _) in results {
            let RuleFinalState::Finished(Some(result)) = state else {
                panic!("isolated scan did not return a finding");
            };
            assert_eq!(result.findings["counter"].0, vec!["1".to_string()]);
        }
    }

    #[tokio::test]
    async fn executes_passive_rule() {
        let rule = PythonRule::from_file("rules/example/http/passive/security_headers.py").unwrap();
        let pair = RequestResponsePair {
            request: Some(HyperRequestWrapper {
                uri: "http://example.test/".to_string(),
                method: "GET".to_string(),
                version: "HTTP/1.1".to_string(),
                headers: HeaderMap::new(),
                body: Vec::new(),
            }),
            response: Some(HyperResponseWrapper {
                status: "200 OK".to_string(),
                version: "HTTP/1.1".to_string(),
                headers: HeaderMap::new(),
                body: Vec::new(),
            }),
            index: 7,
            timestamp: None,
        };

        let (RuleFinalState::Finished(Some(result)), _) = rule.execute(Arc::new(pair)).await else {
            panic!("passive Python rule did not return a finding");
        };
        assert_eq!(result.get_rule_id(), "python-missing-security-headers");
        assert!(result.get_findings().contains_key("missing-security-headers"));
    }
}
