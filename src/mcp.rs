use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
    sync::Arc,
    time::Duration,
};

use rmcp::{
    handler::server::{
        router::tool::ToolRouter,
        wrapper::{Json, Parameters},
    },
    model::{Implementation, ServerCapabilities, ServerConfig},
    schemars, tool, tool_handler, tool_router, ErrorData as McpError, ServerHandler, ServiceExt,
};
use serde::{Deserialize, Serialize};

use crate::{
    audit::{load_rule::load_rules, python::PythonRuleType, AuditRule},
    config::{AuditConfig, Config, McpConfig},
    http_sender::send_request_from_wrapper,
    http_storage::{serializable::SerializableProxyData, HTTPStorage},
    utils::CrusterError,
};

const DEFAULT_PAGE_SIZE: usize = 20;
const MAX_PAGE_SIZE: usize = 100;

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct HistoryListInput {
    #[serde(default)]
    offset: usize,
    limit: Option<usize>,
    method: Option<String>,
    host: Option<String>,
    status: Option<String>,
    #[serde(default)]
    include_bodies: bool,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct HistoryGetInput {
    id: usize,
    #[serde(default = "default_true")]
    include_bodies: bool,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct SendHistoryInput {
    id: usize,
    #[serde(default = "default_redirects")]
    max_redirects: usize,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct AuditRunInput {
    pair_ids: Vec<usize>,
    rule_ids: Option<Vec<String>>,
}

fn default_true() -> bool {
    true
}
fn default_redirects() -> usize {
    5
}

#[derive(Debug, Serialize, schemars::JsonSchema)]
struct ProjectStatus {
    project: String,
    history_file_exists: bool,
    history_records: usize,
    configured_rules: usize,
    dangerous_tools_enabled: bool,
    max_result_bytes: usize,
    request_timeout_seconds: u64,
}

#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
struct HistoryPage {
    items: Vec<serde_json::Value>,
    next_offset: Option<usize>,
}

#[derive(Debug, Serialize, schemars::JsonSchema)]
struct AuditEntry {
    pair_id: usize,
    rule_id: String,
    state: serde_json::Value,
    logs: serde_json::Value,
}

#[derive(Clone)]
struct CrusterMcpServer {
    project: Arc<PathBuf>,
    audit_config: AuditConfig,
    settings: McpConfig,
    tool_router: ToolRouter<Self>,
}

impl CrusterMcpServer {
    fn new(project: PathBuf, audit_config: AuditConfig, settings: McpConfig) -> Self {
        Self {
            project: Arc::new(project),
            audit_config,
            settings,
            tool_router: Self::tool_router(),
        }
    }

    fn history_path(&self) -> PathBuf {
        self.project.join("http.jsonl")
    }

    fn read_history(&self) -> Result<Vec<SerializableProxyData>, McpError> {
        let path = self.history_path();
        if !path.exists() {
            return Ok(Vec::new());
        }
        let file = File::open(&path).map_err(internal_error)?;
        BufReader::new(file)
            .lines()
            .enumerate()
            .map(|(index, line)| {
                let line = line.map_err(internal_error)?;
                serde_json::from_str(&line).map_err(|error| {
                    McpError::internal_error(
                        format!("Cannot parse HTTP history line {}: {}", index + 1, error),
                        None,
                    )
                })
            })
            .collect()
    }

    fn load_audit_rules(&self) -> Result<Vec<Arc<AuditRule>>, McpError> {
        load_rules(&self.audit_config)
            .map_err(|error| McpError::internal_error(error.to_string(), None))
    }

    fn bounded_json<T: Serialize>(&self, value: &T) -> Result<Json<serde_json::Value>, McpError> {
        let size = serde_json::to_vec(value).map_err(internal_error)?.len();
        if size > self.settings.max_result_bytes {
            return Err(McpError::invalid_params(format!(
                "Result is {} bytes and exceeds --max-result-bytes {}. Request fewer records or omit bodies.",
                size, self.settings.max_result_bytes), None));
        }
        Ok(Json(serde_json::to_value(value).map_err(internal_error)?))
    }
}

#[tool_router]
impl CrusterMcpServer {
    #[tool(description = "Return Cruster project state and MCP security limits")]
    fn project_status(&self) -> Result<Json<ProjectStatus>, McpError> {
        let records = self.read_history()?.len();
        let rules = self.load_audit_rules()?.len();
        Ok(Json(ProjectStatus {
            project: self
                .project
                .file_name()
                .and_then(|v| v.to_str())
                .unwrap_or("project")
                .to_string(),
            history_file_exists: self.history_path().exists(),
            history_records: records,
            configured_rules: rules,
            dangerous_tools_enabled: self.settings.allow_dangerous_tools,
            max_result_bytes: self.settings.max_result_bytes,
            request_timeout_seconds: self.settings.request_timeout_seconds,
        }))
    }

    #[tool(
        description = "List persisted HTTP exchanges with bounded pagination and optional filters"
    )]
    fn http_history_list(
        &self,
        Parameters(input): Parameters<HistoryListInput>,
    ) -> Result<Json<serde_json::Value>, McpError> {
        let limit = input.limit.unwrap_or(DEFAULT_PAGE_SIZE).min(MAX_PAGE_SIZE);
        let records: Vec<_> = self
            .read_history()?
            .into_iter()
            .filter(|item| {
                input
                    .method
                    .as_ref()
                    .map_or(true, |v| item.request.method.eq_ignore_ascii_case(v))
                    && input
                        .host
                        .as_ref()
                        .map_or(true, |v| item.request.host.contains(v))
                    && input.status.as_ref().map_or(true, |v| {
                        item.response
                            .as_ref()
                            .map_or(false, |r| r.status.starts_with(v))
                    })
            })
            .collect();
        let mut items = Vec::new();
        for mut item in records.iter().skip(input.offset).take(limit + 1).cloned() {
            if !input.include_bodies {
                item.request.body = None;
                if let Some(response) = item.response.as_mut() {
                    response.body = None;
                }
            }
            items.push(serde_json::to_value(item).map_err(internal_error)?);
        }
        let has_more = items.len() > limit;
        items.truncate(limit);
        let page = HistoryPage {
            items,
            next_offset: has_more.then_some(input.offset + limit),
        };
        self.bounded_json(&page)
    }

    #[tool(description = "Get one persisted HTTP exchange by pair ID")]
    fn http_history_get(
        &self,
        Parameters(input): Parameters<HistoryGetInput>,
    ) -> Result<Json<serde_json::Value>, McpError> {
        let mut item = self
            .read_history()?
            .into_iter()
            .find(|item| item.index == input.id)
            .ok_or_else(|| {
                McpError::invalid_params(format!("HTTP pair {} was not found", input.id), None)
            })?;
        if !input.include_bodies {
            item.request.body = None;
            if let Some(response) = item.response.as_mut() {
                response.body = None;
            }
        }
        let value = serde_json::to_value(item).map_err(internal_error)?;
        if serde_json::to_vec(&value).map_err(internal_error)?.len()
            > self.settings.max_result_bytes
        {
            return Err(McpError::invalid_params("HTTP exchange exceeds the configured result limit; retry with include_bodies=false", None));
        }
        Ok(Json(value))
    }

    #[tool(description = "List metadata for configured Python audit rules")]
    fn audit_rules_list(&self) -> Result<Json<Vec<serde_json::Value>>, McpError> {
        let rules = self
            .load_audit_rules()?
            .into_iter()
            .map(|rule| serde_json::json!({"id": rule.get_id()}))
            .collect();
        Ok(Json(rules))
    }

    #[tool(
        description = "Run configured Python audit rules for persisted HTTP pairs. Active rules require --allow-dangerous-tools"
    )]
    async fn audit_python_run(
        &self,
        Parameters(input): Parameters<AuditRunInput>,
    ) -> Result<Json<Vec<AuditEntry>>, McpError> {
        let mut storage = HTTPStorage::default();
        storage
            .load(
                self.history_path()
                    .to_str()
                    .ok_or_else(|| McpError::internal_error("Non-UTF-8 project path", None))?,
            )
            .map_err(internal_error)?;
        let rules = self.load_audit_rules()?;
        let mut output = Vec::new();
        for pair_id in input.pair_ids {
            let pair = storage.get_by_id(pair_id).cloned().ok_or_else(|| {
                McpError::invalid_params(format!("HTTP pair {} was not found", pair_id), None)
            })?;
            for rule in &rules {
                if input
                    .rule_ids
                    .as_ref()
                    .map_or(false, |ids| !ids.iter().any(|id| id == rule.get_id()))
                {
                    continue;
                }
                if matches!(rule.as_ref(), AuditRule::Python(r) if r.rule_type() == &PythonRuleType::Active)
                    && !self.settings.allow_dangerous_tools
                {
                    return Err(McpError::invalid_params(
                        format!(
                            "Rule '{}' is active; restart MCP with --allow-dangerous-tools",
                            rule.get_id()
                        ),
                        None,
                    ));
                }
                let (state, logs) = rule.execute(Arc::new(pair.clone())).await;
                output.push(AuditEntry {
                    pair_id,
                    rule_id: rule.get_id().to_string(),
                    state: serde_json::to_value(state).map_err(internal_error)?,
                    logs: serde_json::to_value(logs).map_err(internal_error)?,
                });
            }
        }
        if serde_json::to_vec(&output).map_err(internal_error)?.len()
            > self.settings.max_result_bytes
        {
            return Err(McpError::invalid_params(
                "Audit output exceeds the configured result limit",
                None,
            ));
        }
        Ok(Json(output))
    }

    #[tool(
        description = "Resend a persisted HTTP request. Disabled unless MCP starts with --allow-dangerous-tools"
    )]
    async fn http_request_send(
        &self,
        Parameters(input): Parameters<SendHistoryInput>,
    ) -> Result<Json<serde_json::Value>, McpError> {
        if !self.settings.allow_dangerous_tools {
            return Err(McpError::invalid_params(
                "Dangerous MCP tools are disabled; restart with --allow-dangerous-tools",
                None,
            ));
        }
        let item = self
            .read_history()?
            .into_iter()
            .find(|item| item.index == input.id)
            .ok_or_else(|| {
                McpError::invalid_params(format!("HTTP pair {} was not found", input.id), None)
            })?;
        let request = item.request.try_into().map_err(internal_error)?;
        let sent = tokio::time::timeout(
            Duration::from_secs(self.settings.request_timeout_seconds),
            send_request_from_wrapper(&request, input.max_redirects),
        )
        .await
        .map_err(|_| McpError::internal_error("HTTP request timed out", None))?
        .map_err(internal_error)?;
        let value = serde_json::json!({"response": format!("{}", sent.0), "redirect_limit_reached": sent.1});
        Ok(Json(value))
    }
}

#[tool_handler(router = self.tool_router)]
impl ServerHandler for CrusterMcpServer {
    fn get_info(&self) -> ServerConfig {
        ServerConfig::new(ServerCapabilities::builder().enable_tools().build())
            .with_server_info(Implementation::new("cruster", env!("CARGO_PKG_VERSION")))
            .with_instructions("Operate on the configured persisted Cruster project. Network sends and active rules are permission-gated. Prefer bounded history queries without bodies.")
    }
}

fn internal_error(error: impl ToString) -> McpError {
    McpError::internal_error(error.to_string(), None)
}

pub(crate) async fn serve(
    config: Config,
    audit_config: AuditConfig,
    settings: McpConfig,
) -> Result<(), CrusterError> {
    let project = config.project.ok_or_else(|| {
        CrusterError::ConfigError("MCP mode requires --project or a project in config".to_string())
    })?;
    let project = std::fs::canonicalize(&project).map_err(|error| {
        CrusterError::ConfigError(format!("Cannot open MCP project '{}': {}", project, error))
    })?;
    let service = CrusterMcpServer::new(project, audit_config, settings)
        .serve(rmcp::transport::stdio())
        .await
        .map_err(|error| {
            CrusterError::UndefinedError(format!("Cannot start MCP stdio server: {}", error))
        })?;
    service
        .waiting()
        .await
        .map_err(|error| CrusterError::UndefinedError(format!("MCP server failed: {}", error)))?;
    Ok(())
}
