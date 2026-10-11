use std::{collections::HashMap, fs, path::PathBuf, sync::Arc};

use base64;
use http::{header::HeaderName, HeaderMap, HeaderValue};
use rustpython::InterpreterConfig;
use rustpython_vm::{builtins::PyStrRef, convert::TryFromObject, VirtualMachine};
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

impl PythonRule {
    pub(crate) fn from_file(path: &str) -> Result<Self, AuditError> {
        let source = fs::read_to_string(path)?;
        let metadata_json = run_python(path, &source, None, None, true, false)?;
        let metadata: PythonMetadata = serde_json::from_str(&metadata_json).map_err(|err| {
            AuditError(format!(
                "Invalid metadata in Python rule '{}': {}",
                path, err
            ))
        })?;
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

    pub(crate) async fn execute(&self, pair: Arc<RequestResponsePair>) -> RuleFinalState {
        let Some(request) = pair.request.as_ref() else {
            return RuleFinalState::Skipped(format!("Pair {} has no request", pair.index));
        };
        let Some(response) = pair.response.as_ref() else {
            return RuleFinalState::Skipped(format!("Pair {} has no response", pair.index));
        };

        let request = PythonRequest::from(request);
        let response = PythonResponse::from(response);
        let source = match fs::read_to_string(&self.path) {
            Ok(source) => source,
            Err(err) => {
                return RuleFinalState::Failed(format!(
                    "Cannot read Python rule '{}': {}",
                    self.path.display(),
                    err
                ))
            }
        };
        let path = self.path.to_string_lossy().to_string();
        let active = self.rule_type == PythonRuleType::Active;

        let execution = tokio::task::spawn_blocking(move || {
            run_python(&path, &source, Some(request), Some(response), false, active)
        })
        .await;
        let json = match execution {
            Ok(Ok(json)) => json,
            Ok(Err(err)) => return RuleFinalState::Failed(err.to_string()),
            Err(err) => {
                return RuleFinalState::Failed(format!("Python rule worker failed: {}", err))
            }
        };
        let python_findings: Vec<PythonFinding> = match serde_json::from_str(&json) {
            Ok(findings) => findings,
            Err(err) => {
                return RuleFinalState::Failed(format!(
                    "Rule '{}' returned invalid findings: {}",
                    self.id, err
                ))
            }
        };
        if python_findings.is_empty() {
            return RuleFinalState::Finished(None);
        }

        let mut findings = HashMap::with_capacity(python_findings.len());
        for finding in python_findings {
            let exchanges = finding
                .exchanges
                .into_iter()
                .map(|exchange| SerializableSendResultEntry {
                    request: HyperRequestWrapper::try_from(exchange.request)
                        .map(|v| v.to_string())
                        .unwrap_or_default(),
                    payloads: exchange.payloads,
                    response: HyperResponseWrapper::try_from(exchange.response)
                        .map(|v| v.to_string())
                        .unwrap_or_default(),
                })
                .collect();
            findings.insert(finding.name, (finding.extracted, exchanges));
        }

        RuleFinalState::Finished(Some(RuleResult {
            id: 0,
            rule_id: self.id.clone(),
            about: self.about.clone(),
            protocol: "http".to_string(),
            r#type: match self.rule_type {
                PythonRuleType::Active => "active",
                PythonRuleType::Passive => "passive",
            }
            .to_string(),
            pair_index: pair.index,
            severity: self.severity.clone(),
            findings,
            initial_request: pair.request.as_ref().unwrap().to_string(),
            initial_response: pair.response.as_ref().unwrap().to_string(),
        }))
    }
}

fn run_python(
    path: &str,
    source: &str,
    request: Option<PythonRequest>,
    response: Option<PythonResponse>,
    metadata_only: bool,
    allow_send: bool,
) -> Result<String, AuditError> {
    let runtime = tokio::runtime::Handle::try_current().ok();
    let runtime_for_send = runtime.clone();
    let interpreter = InterpreterConfig::new().init_stdlib().interpreter();

    interpreter.enter(|vm| {
        let scope = vm.new_scope_with_builtins();
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
                let wrapper = HyperRequestWrapper::try_from(request.clone())
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
                let python_response = PythonResponse::from(&response);
                serde_json::to_string(&python_response)
                    .map_err(|err| vm.new_runtime_error(err.to_string()))
            },
        );
        scope
            .globals
            .set_item("__cruster_send", send.into(), vm)
            .map_err(|err| python_error(vm, err))?;
        vm.run_code_string(scope.clone(), BOOTSTRAP, "<cruster>".to_string())
            .map_err(|err| python_error(vm, err))?;
        vm.run_code_string(scope.clone(), source, path.to_string())
            .map_err(|err| python_error(vm, err))?;

        let expression = if metadata_only {
            "__cruster_json.dumps(metadata)".to_string()
        } else {
            let request_json = serde_json::to_string(&request.unwrap())?;
            let response_json = serde_json::to_string(&response.unwrap())?;
            format!(
                "__cruster_json.dumps(scan(__cruster_request({}), __cruster_response({})))",
                python_literal(&request_json),
                python_literal(&response_json)
            )
        };
        let value = vm
            .run_block_expr(scope, &expression)
            .map_err(|err| python_error(vm, err))?;
        String::try_from_object(vm, value).map_err(|err| python_error(vm, err))
    })
}

fn python_error(
    vm: &VirtualMachine,
    error: rustpython_vm::builtins::PyBaseExceptionRef,
) -> AuditError {
    let mut output = String::new();
    let _ = vm.write_exception(&mut output, &error);
    AuditError(output.trim().to_string())
}

fn python_literal(value: &str) -> String {
    serde_json::to_string(value).unwrap_or_else(|_| "\"\"".to_string())
}

const BOOTSTRAP: &str = r#"
import json as __cruster_json

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
"#;

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

    #[test]
    fn loads_python_metadata() {
        let rule = PythonRule::from_file("rules/python/passive/security_headers.py").unwrap();
        assert_eq!(rule.get_id(), "python-missing-security-headers");
        assert_eq!(rule.rule_type(), &PythonRuleType::Passive);
    }

    #[tokio::test]
    async fn executes_passive_rule() {
        let rule = PythonRule::from_file("rules/python/passive/security_headers.py").unwrap();
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

        let RuleFinalState::Finished(Some(result)) = rule.execute(Arc::new(pair)).await else {
            panic!("passive Python rule did not return a finding");
        };
        assert_eq!(result.get_rule_id(), "python-missing-security-headers");
        assert!(result.get_findings().contains_key("missing-security-headers"));
    }
}
