//! CLI-equivalent audit execution, testing and persisted findings.
//! Python scans are not cancellable; request timeout bounds explicit HTTP fetches only.
//! Persistence refuses overwrites and interrupted runs may leave partial JSONL.
use std::{
    collections::HashSet,
    fs::{self, File, OpenOptions},
    io::{BufReader, Write},
    path::{Path, PathBuf},
    sync::Arc,
};

use regex::Regex;
use rmcp::{handler::server::wrapper::Json, schemars, ErrorData as McpError};
use serde::Deserialize;
use serde_json::{json, Value};

use super::{internal_error, CrusterMcpServer, DEFAULT_PAGE_SIZE, MAX_PAGE_SIZE};
use crate::{
    audit::{
        load_rule::load_rules,
        python::{PythonRule, PythonRuleType},
        AuditRule, RuleFinalState,
    },
    config::{AuditConfig, AuditEntities},
    cruster_proxy::request_response::{HyperRequestWrapper, HyperResponseWrapper},
    http_storage::RequestResponsePair,
};

fn invalid(message: impl ToString) -> McpError {
    McpError::invalid_params(message.to_string(), None)
}

#[derive(Debug, Default, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(super) struct AuditSelection {
    /// Persistent pair IDs, not positions. Intersects range and filters when supplied.
    pair_ids: Option<Vec<usize>>,
    /// CLI HTTP range: a, n (first n), n-m (1-based inclusive), -n (last n), iN or Ni (pair ID).
    range: Option<String>,
    /// Additional regexes; ORed with each other and configured HTTP filters.
    #[serde(default)]
    filters: Vec<String>,
}

#[derive(Debug, Default, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(super) struct AuditEntitiesInput {
    #[serde(default)]
    ids: Vec<String>,
    #[serde(default)]
    tags: Vec<String>,
    /// Regex filters on rule filenames, NOT paths to open.
    #[serde(default)]
    paths: Vec<String>,
}

fn extend_entities(target: &mut Option<AuditEntities>, input: AuditEntitiesInput) {
    let target = target.get_or_insert_with(AuditEntities::default);
    target.ids.extend(input.ids);
    target.tags.extend(input.tags);
    target.paths.extend(input.paths);
}

#[derive(Debug, Default, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(super) struct AuditOptions {
    passive: Option<bool>,
    active: Option<bool>,
    /// Appended to configured selectors, like CLI --ids/--tags.
    include: Option<AuditEntitiesInput>,
    exclude: Option<AuditEntitiesInput>,
    /// Replace configured rule paths. Only project/configured roots are allowed.
    rule_paths: Option<Vec<String>>,
    #[serde(default)]
    add_paths: Vec<String>,
    /// Clear inherited include/exclude selectors before applying this request's selectors.
    #[serde(default)]
    reset_selectors: bool,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(super) struct AuditExecuteInput {
    #[serde(default)]
    selection: AuditSelection,
    #[serde(default)]
    options: AuditOptions,
    /// Concurrent rules; each rule processes its selected pairs with the domain batch API.
    tasks: Option<usize>,
    /// Strict basename, without audit_ prefix or .jsonl suffix. Defaults to configured audit name.
    name: Option<String>,
    /// Persist findings in CLI-compatible audit_NAME.jsonl. Existing names are never overwritten.
    #[serde(default)]
    persist: bool,
    /// Return execution states and Python logs as space permits. Findings are always counted.
    /// Omitted means inherit the configured verbosity.
    verbose: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, schemars::JsonSchema)]
#[serde(tag = "source", rename_all = "snake_case", deny_unknown_fields)]
pub(super) enum AuditRuleInput {
    Configured {
        id: String,
    },
    /// A file under the selected project or a configured rule root.
    File {
        path: String,
    },
    /// Arbitrary Python execution. Requires allow_dangerous_tools, even for passive metadata.
    Inline {
        python: String,
    },
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(tag = "source", rename_all = "snake_case", deny_unknown_fields)]
pub(super) enum AuditPairInput {
    Saved {
        id: usize,
    },
    /// Raw request with either an inline response or an explicitly requested live response.
    Raw {
        request: String,
        /// Absolute connection URL; the request line supplies path and query.
        url: String,
        /// Complete raw response. Required unless fetch_response=true.
        response: Option<String>,
        /// Sends the raw request to obtain a response; requires allow_dangerous_tools.
        #[serde(default)]
        fetch_response: bool,
    },
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(super) struct AuditTestInput {
    rule: AuditRuleInput,
    pair: AuditPairInput,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(super) struct AuditDebugInput {
    rule: AuditRuleInput,
    /// Omit to debug against all saved pairs, like CLI debug-rule without -i.
    pair: Option<AuditPairInput>,
    #[serde(default)]
    offset: usize,
    limit: Option<usize>,
}

#[derive(Debug, Default, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub(super) enum AuditSort {
    #[default]
    File,
    RuleId,
    Severity,
    SeverityRuleId,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(super) struct AuditResultsInput {
    name: String,
    #[serde(default)]
    offset: usize,
    limit: Option<usize>,
    id: Option<usize>,
    pair_ids: Option<Vec<usize>>,
    rule_ids: Option<Vec<String>>,
    severities: Option<Vec<String>>,
    /// Regex against the complete serialized finding before projection.
    filter: Option<String>,
    #[serde(default)]
    sort: AuditSort,
    #[serde(default)]
    include_data: bool,
    #[serde(default)]
    include_initial: bool,
    #[serde(default)]
    include_bodies: bool,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(super) struct AuditResultContentInput {
    name: String,
    /// Finding ID, not line number.
    id: usize,
    /// UTF-8 byte offset into canonical JSON of the entire finding. Empty means EOF.
    #[serde(default)]
    offset: usize,
    length: Option<usize>,
}

fn validate_name(name: &str) -> Result<(), McpError> {
    if name.is_empty()
        || name.len() > 128
        || name == "."
        || name == ".."
        || !name
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"._-".contains(&b))
    {
        return Err(invalid("Audit name must be a basename of 1..128 ASCII letters, digits, '.', '_' or '-' (not . or ..)"));
    }
    Ok(())
}

fn select_range(ids: &[usize], range: Option<&str>) -> Result<HashSet<usize>, McpError> {
    let range = crate::cli::http::show::parse_range(range.unwrap_or("a"))
        .map_err(|error| invalid(Into::<String>::into(error)))?;
    let positions = range.select_positions(ids.len(), None, |position| Some(ids[position]));
    if positions.is_empty() && range.selects_id(None) {
        return Err(invalid("HTTP pair ID was not found"));
    }
    Ok(positions.into_iter().map(|position| ids[position]).collect())
}

/// Remove a uniquely-created inline file even on validation/execution failure.
struct InlineFile(PathBuf);
impl Drop for InlineFile {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.0);
    }
}

impl CrusterMcpServer {
    pub(super) async fn debug_audit_rule(
        &self,
        input: AuditDebugInput,
    ) -> Result<Json<Value>, McpError> {
        if let Some(pair) = input.pair {
            return self
                .test_audit_rule(AuditTestInput {
                    rule: input.rule,
                    pair,
                })
                .await;
        }
        let records = self.read_history()?;
        let total = records.len();
        let limit = input
            .limit
            .unwrap_or(DEFAULT_PAGE_SIZE)
            .clamp(1, MAX_PAGE_SIZE);
        let mut items = Vec::new();
        let mut used = 0;
        let mut truncated = false;
        for record in records.into_iter().skip(input.offset).take(limit) {
            let result = self
                .test_audit_rule(AuditTestInput {
                    rule: input.rule.clone(),
                    pair: AuditPairInput::Saved { id: record.index },
                })
                .await?;
            let size = serde_json::to_vec(&result.0).map_err(internal_error)?.len();
            if used + size + 256 > self.settings.max_result_bytes {
                // This pair has executed: advance past it, never cause an implicit resend.
                items.push(json!({"pair_id":record.index,"output_omitted":true}));
                truncated = true;
                break;
            }
            used += size;
            items.push(result.0);
        }
        let end = input.offset.saturating_add(items.len());
        self.bounded_json(&json!({"items":items,"total":total,"next_offset":(end < total).then_some(end),"output_truncated":truncated}))
    }

    fn audit_roots(&self) -> Result<Vec<PathBuf>, McpError> {
        let mut roots = vec![self.project.canonicalize().map_err(internal_error)?];
        for path in &self.audit_config.rules {
            roots.push(Path::new(path).canonicalize().map_err(internal_error)?);
        }
        Ok(roots)
    }

    fn audit_safe_path(&self, path: &Path) -> Result<PathBuf, McpError> {
        let candidate = if path.is_absolute() {
            path.to_path_buf()
        } else {
            self.project.join(path)
        };
        let candidate = candidate.canonicalize().map_err(invalid)?;
        if !self
            .audit_roots()?
            .iter()
            .any(|root| candidate == *root || (root.is_dir() && candidate.starts_with(root)))
        {
            return Err(invalid(
                "Rule path is outside the selected project and configured rule roots",
            ));
        }
        Ok(candidate)
    }

    fn audit_config_for(&self, options: AuditOptions) -> Result<AuditConfig, McpError> {
        let mut config = self.audit_config.clone();
        if let Some(value) = options.passive {
            config.passive = value;
        }
        if let Some(value) = options.active {
            config.active = value;
        }
        if config.active && !self.settings.allow_dangerous_tools {
            return Err(invalid("Active audit requires --allow-dangerous-tools; use active=false for passive-only execution"));
        }
        if options.reset_selectors {
            config.include = None;
            config.exclude = None;
        }
        if let Some(input) = options.include {
            extend_entities(&mut config.include, input);
        }
        if let Some(input) = options.exclude {
            extend_entities(&mut config.exclude, input);
        }
        if let Some(paths) = options.rule_paths {
            config.rules = paths
                .iter()
                .map(|p| {
                    self.audit_safe_path(Path::new(p))
                        .map(|p| p.to_string_lossy().into_owned())
                })
                .collect::<Result<_, _>>()?;
        }
        for path in options.add_paths {
            config.rules.push(
                self.audit_safe_path(Path::new(&path))?
                    .to_string_lossy()
                    .into_owned(),
            );
        }
        // Resolve all descendants before calling the shared loader: a project rule
        // directory must not use nested symlinks to read arbitrary host files.
        let mut visited = HashSet::new();
        let mut files = Vec::new();
        let mut pending: Vec<PathBuf> = config.rules.iter().map(PathBuf::from).collect();
        while let Some(path) = pending.pop() {
            let path = path.canonicalize().map_err(invalid)?;
            self.audit_safe_path(&path)?;
            if !visited.insert(path.clone()) {
                continue;
            }
            if path.is_dir() {
                for entry in fs::read_dir(path).map_err(internal_error)? {
                    pending.push(entry.map_err(internal_error)?.path());
                }
            } else if path.extension().and_then(|s| s.to_str()) == Some("py") {
                files.push(
                    path.to_str()
                        .ok_or_else(|| invalid("Non UTF-8 rule path"))?
                        .to_owned(),
                );
            }
        }
        files.sort();
        config.rules = files;
        Ok(config)
    }

    fn audit_pairs(
        &self,
        selection: AuditSelection,
    ) -> Result<Vec<Arc<RequestResponsePair>>, McpError> {
        let records = self.read_history()?;
        let ids: Vec<_> = records.iter().map(|p| p.index).collect();
        if let Some(requested) = &selection.pair_ids {
            for id in requested {
                if !ids.contains(id) {
                    return Err(invalid(format!("HTTP pair {id} was not found")));
                }
            }
        }
        let selected = select_range(&ids, selection.range.as_deref())?;
        let mut patterns = self.audit_config.filter.clone().unwrap_or_default();
        patterns.extend(selection.filters);
        let filters = crate::audit::operations::compile_pair_filters(&AuditConfig {
            filter: Some(patterns),
            ..Default::default()
        })
        .map_err(|e| invalid(e.to_string()))?;
        let mut pairs = Vec::new();
        for record in records {
            if !selected.contains(&record.index)
                || selection
                    .pair_ids
                    .as_ref()
                    .is_some_and(|ids| !ids.contains(&record.index))
            {
                continue;
            }
            let pair: RequestResponsePair = record.try_into().map_err(internal_error)?;
            pairs.push(Arc::new(pair));
        }
        crate::audit::operations::filter_pairs(&mut pairs, &filters);
        Ok(pairs)
    }

    fn audit_result_path(&self, name: &str) -> Result<PathBuf, McpError> {
        validate_name(name)?;
        let path = self.project.join(format!("audit_{name}.jsonl"));
        if let Ok(metadata) = fs::symlink_metadata(&path) {
            if metadata.file_type().is_symlink() || !metadata.is_file() {
                return Err(invalid(
                    "Audit result must be a regular file, not a symlink",
                ));
            }
        }
        Ok(path)
    }

    /// CLI audit run equivalent, with optional persistence and bounded structured output.
    pub(super) async fn execute_audit(
        &self,
        input: AuditExecuteInput,
    ) -> Result<Json<Value>, McpError> {
        let config = self.audit_config_for(input.options)?;
        let verbose = input.verbose.unwrap_or(config.verbose.unwrap_or(false));
        let tasks = input.tasks.or(config.tasks).unwrap_or_else(num_cpus::get);
        if tasks == 0 || tasks > 256 {
            return Err(invalid("tasks must be between 1 and 256"));
        }
        let name = input.name.or(config.name.clone());
        if let Some(name) = &name {
            validate_name(name)?;
        }
        let rules = load_rules(&config).map_err(|e| internal_error(e.to_string()))?;
        let pairs = self.audit_pairs(input.selection)?;
        // create_new prevents overwriting old runs and never follows a result symlink.
        let mut output_file = if input.persist {
            let name = name
                .as_deref()
                .ok_or_else(|| invalid("Persistence requires an audit name"))?;
            Some(
                OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open(self.audit_result_path(name)?)
                    .map_err(internal_error)?,
            )
        } else {
            None
        };
        let rule_count = rules.len();
        let pair_count = pairs.len();
        let (mut findings, mut failed, mut skipped, mut executions) =
            (0usize, 0usize, 0usize, 0usize);
        let mut entries = Vec::new();
        let mut omitted = 0usize;
        // Leave room for the summary envelope. Execution is not cancelled when a
        // response fills up: every finding still reaches the persisted JSONL.
        let budget = self.settings.max_result_bytes.saturating_sub(1024);
        let mut used = 0usize;
        crate::audit::execution::execute_scan(rules, pairs, tasks, |message| {
            let crate::audit::execution::WorkerToMainMsg::Result(result) = message else {
                return Ok::<_, crate::audit::AuditError>(());
            };
            let crate::audit::execution::ExecutionResult {
                pair_id,
                rule_id,
                mut state,
                logs,
            } = result;
            executions += 1;
            match &mut state {
                RuleFinalState::Finished(Some(result)) => {
                    result.set_id(findings);
                    findings += 1;
                    if let Some(file) = &mut output_file {
                        serde_json::to_writer(&mut *file, result)
                            .map_err(crate::audit::AuditError::from)?;
                        file.write_all(b"\n")
                            .map_err(crate::audit::AuditError::from)?;
                    }
                }
                RuleFinalState::Failed(_) => failed += 1,
                RuleFinalState::Skipped(_) => skipped += 1,
                _ => {}
            }
            if verbose || matches!(&state, RuleFinalState::Finished(Some(_))) {
                let entry = if verbose {
                    json!({"pair_id":pair_id,"rule_id":rule_id,"state":state,"logs":logs})
                } else {
                    json!({"pair_id":pair_id,"rule_id":rule_id,"state":state})
                };
                let size = serde_json::to_vec(&entry)
                    .map_err(crate::audit::AuditError::from)?
                    .len()
                    + 1;
                if used + size <= budget {
                    entries.push(entry);
                    used += size;
                } else {
                    omitted += 1;
                }
            }
            Ok(())
        })
        .await
        .map_err(|e| internal_error(e.to_string()))?;
        if let Some(file) = &mut output_file {
            file.flush().map_err(internal_error)?;
        }
        self.bounded_json(
            &json!({"name":name,"persisted":input.persist,"rules":rule_count,
            "pairs":pair_count,"executions":executions,"findings":findings,"failed":failed,
            "skipped":skipped,"entries":entries,"omitted_entries":omitted}),
        )
    }

    /// Both test and debug-rule wrappers delegate here. Logs are returned without
    /// changing global logging, using a debugger, or writing to protocol stdout.
    pub(super) async fn test_audit_rule(
        &self,
        input: AuditTestInput,
    ) -> Result<Json<Value>, McpError> {
        let mut inline_file = None;
        let rule = match input.rule {
            AuditRuleInput::Configured { id } => {
                // Validate configured descendants too, not only caller-provided paths.
                let mut config = self.audit_config_for(AuditOptions {
                    passive: Some(true),
                    active: Some(false),
                    reset_selectors: true,
                    ..Default::default()
                })?;
                config.active = true;
                config.passive = true;
                config.include = Some(AuditEntities {
                    ids: vec![id.clone()],
                    ..Default::default()
                });
                config.exclude = None;
                let mut rules = load_rules(&config).map_err(|e| internal_error(e.to_string()))?;
                rules.retain(|r| r.get_id() == id);
                if rules.len() != 1 {
                    return Err(invalid(
                        "Configured rule ID must resolve to exactly one rule",
                    ));
                }
                rules.remove(0)
            }
            AuditRuleInput::File { path } => {
                let path = self.audit_safe_path(Path::new(&path))?;
                if path.extension().and_then(|s| s.to_str()) != Some("py") {
                    return Err(invalid("Only Python rules are supported"));
                }
                Arc::new(AuditRule::Python(
                    PythonRule::from_file(
                        path.to_str()
                            .ok_or_else(|| invalid("Non UTF-8 rule path"))?,
                    )
                    .map_err(|e| invalid(e.to_string()))?,
                ))
            }
            AuditRuleInput::Inline { python } => {
                if !self.settings.allow_dangerous_tools {
                    return Err(invalid("Inline Python requires --allow-dangerous-tools"));
                }
                if python.len() > 1024 * 1024 {
                    return Err(invalid("Inline rule exceeds 1 MiB"));
                }
                let path = self
                    .project
                    .join(format!(".mcp-audit-{:032x}.py", rand::random::<u128>()));
                let mut file = OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open(&path)
                    .map_err(internal_error)?;
                inline_file = Some(InlineFile(path.clone()));
                file.write_all(python.as_bytes()).map_err(internal_error)?;
                drop(file);
                Arc::new(AuditRule::Python(
                    PythonRule::from_file(
                        path.to_str()
                            .ok_or_else(|| invalid("Non UTF-8 project path"))?,
                    )
                    .map_err(|e| invalid(e.to_string()))?,
                ))
            }
        };
        if matches!(rule.as_ref(), AuditRule::Python(r) if r.rule_type() == &PythonRuleType::Active)
            && !self.settings.allow_dangerous_tools
        {
            return Err(invalid("Active rule requires --allow-dangerous-tools"));
        }
        let pair = match input.pair {
            AuditPairInput::Saved { id } => {
                // An explicit debug target is not subject to batch HTTP filters.
                let record = self
                    .read_history()?
                    .into_iter()
                    .find(|p| p.index == id)
                    .ok_or_else(|| invalid(format!("HTTP pair {id} was not found")))?;
                let pair: RequestResponsePair = record.try_into().map_err(internal_error)?;
                Arc::new(pair)
            }
            AuditPairInput::Raw {
                request,
                url,
                response,
                fetch_response,
            } => {
                if response.is_some() == fetch_response {
                    return Err(invalid(
                        "Supply exactly one of response or fetch_response=true",
                    ));
                }
                if fetch_response && !self.settings.allow_dangerous_tools {
                    return Err(invalid(
                        "Fetching a raw request response requires --allow-dangerous-tools",
                    ));
                }
                // Reuse the shared MCP request edit validation/parser without making a repeater.
                let edit: super::repeater::RequestEdit =
                    serde_json::from_value(json!({"raw_request":request})).map_err(invalid)?;
                let mut wrapper = HyperRequestWrapper {
                    uri: url,
                    method: "GET".into(),
                    version: "HTTP/1.1".into(),
                    headers: http::HeaderMap::new(),
                    body: Vec::new(),
                };
                edit.apply(&mut wrapper)?;
                let response = if let Some(response) = response {
                    parse_response(&response)?
                } else {
                    self.send_repeater_request(&wrapper, 0).await?.0
                };
                Arc::new(RequestResponsePair {
                    request: Some(wrapper),
                    response: Some(response),
                    index: 0,
                    timestamp: None,
                })
            }
        };
        let crate::audit::execution::ExecutionResult { state, logs, .. } =
            crate::audit::execution::execute_one(&rule, pair.clone()).await;
        drop(inline_file);
        self.bounded_json(
            &json!({"pair_id":pair.index,"rule_id":rule.get_id(),"state":state,"logs":logs}),
        )
    }

    pub(super) fn audit_results(&self, input: AuditResultsInput) -> Result<Json<Value>, McpError> {
        let filter = input
            .filter
            .as_deref()
            .map(Regex::new)
            .transpose()
            .map_err(invalid)?;
        let file = File::open(self.audit_result_path(&input.name)?).map_err(internal_error)?;
        let query = crate::audit::operations::FindingQuery {
            id: input.id,
            pair_ids: input.pair_ids,
            rule_ids: input.rule_ids,
            severities: input.severities,
            filter,
            sort_ruleid: matches!(input.sort, AuditSort::RuleId | AuditSort::SeverityRuleId),
            sort_severity: matches!(input.sort, AuditSort::Severity | AuditSort::SeverityRuleId),
        };
        let results = crate::audit::operations::query_findings(BufReader::new(file), query)
            .map_err(internal_error)?
            .collect::<std::io::Result<Vec<_>>>()
            .map_err(internal_error)?;
        let limit = input
            .limit
            .unwrap_or(DEFAULT_PAGE_SIZE)
            .clamp(1, MAX_PAGE_SIZE);
        let total = results.len();
        let urls = crate::audit::operations::request_urls(self.read_history()?);
        let mut items = Vec::new();
        for result in results.into_iter().skip(input.offset).take(limit) {
            let mut value = serde_json::to_value(&result).map_err(internal_error)?;
            project_result(
                &mut value,
                input.include_data,
                input.include_initial,
                input.include_bodies,
            );
            value["url"] = urls
                .get(&result.get_pair_index())
                .map_or(Value::Null, |url| json!(url));
            items.push(value);
            let next = input.offset.saturating_add(items.len());
            let probe =
                json!({"items":items,"total":total,"next_offset":(next < total).then_some(next)});
            if serde_json::to_vec(&probe).map_err(internal_error)?.len()
                > self.settings.max_result_bytes
            {
                items.pop();
                if items.is_empty() {
                    return Err(invalid("Finding exceeds result limit; omit data or use audit_result_content for chunked access"));
                }
                break;
            }
        }
        let next = input.offset.saturating_add(items.len());
        self.bounded_json(
            &json!({"items":items,"total":total,"next_offset":(next < total).then_some(next)}),
        )
    }

    pub(super) fn audit_result_content(
        &self,
        input: AuditResultContentInput,
    ) -> Result<Json<Value>, McpError> {
        let file = File::open(self.audit_result_path(&input.name)?).map_err(internal_error)?;
        let query = crate::audit::operations::FindingQuery {
            id: Some(input.id),
            ..Default::default()
        };
        for result in crate::audit::operations::query_findings(BufReader::new(file), query)
            .map_err(internal_error)?
        {
            let result = result.map_err(internal_error)?;
            let text = serde_json::to_string(&result).map_err(internal_error)?;
            if input.offset > text.len() || !text.is_char_boundary(input.offset) {
                return Err(invalid(
                    "offset must be a UTF-8 boundary within the finding JSON",
                ));
            }
            let max = self.settings.max_result_bytes.saturating_sub(256) / 6;
            if max == 0 {
                return Err(invalid("max_result_bytes is too small for chunked content"));
            }
            let length = input.length.unwrap_or(4096).min(max);
            if length == 0 {
                return Err(invalid("length must be positive"));
            }
            let mut end = input.offset.saturating_add(length).min(text.len());
            while !text.is_char_boundary(end) {
                end -= 1;
            }
            if end == input.offset && end < text.len() {
                return Err(invalid("length is too small for the next UTF-8 character"));
            }
            return self.bounded_json(&json!({"id":input.id,"offset":input.offset,"total_bytes":text.len(),
                "content":&text[input.offset..end],"next_offset":(end < text.len()).then_some(end)}));
        }
        Err(invalid("Audit finding ID was not found"))
    }
}

fn parse_response(raw: &str) -> Result<HyperResponseWrapper, McpError> {
    let (head, body) = raw
        .split_once("\r\n\r\n")
        .filter(|(head, _)| !head.contains("\n\n"))
        .or_else(|| raw.split_once("\n\n"))
        .ok_or_else(|| invalid("Raw response must have a blank line after headers"))?;
    let mut lines = head.lines();
    let mut status = lines.next().unwrap_or_default().split_whitespace();
    let version = status
        .next()
        .ok_or_else(|| invalid("Missing response version"))?;
    if !matches!(
        version,
        "HTTP/0.9" | "HTTP/1.0" | "HTTP/1.1" | "HTTP/2" | "HTTP/2.0"
    ) {
        return Err(invalid("Unsupported response HTTP version"));
    }
    let status = status
        .next()
        .ok_or_else(|| invalid("Missing response status"))?;
    http::StatusCode::from_bytes(status.as_bytes()).map_err(invalid)?;
    let mut headers = http::HeaderMap::new();
    for line in lines {
        let (name, value) = line
            .split_once(':')
            .ok_or_else(|| invalid("Malformed response header"))?;
        headers.append(
            name.parse::<http::header::HeaderName>().map_err(invalid)?,
            http::HeaderValue::from_str(value.trim_matches([' ', '\t'])).map_err(invalid)?,
        );
    }
    Ok(HyperResponseWrapper {
        status: status.into(),
        version: version.into(),
        headers,
        body: body.as_bytes().to_vec(),
    })
}

fn headers_only(message: &mut Value) {
    if let Some(text) = message.as_str() {
        let text = text
            .split_once("\r\n\r\n")
            .filter(|(head, _)| !head.contains("\n\n"))
            .map(|(head, _)| head)
            .or_else(|| text.split_once("\n\n").map(|(head, _)| head))
            .unwrap_or(text);
        *message = Value::String(text.to_owned());
    }
}

fn project_result(value: &mut Value, data: bool, initial: bool, bodies: bool) {
    if let Some(findings) = value["findings"].as_object_mut() {
        for finding in findings.values_mut() {
            if !data {
                finding[1] = json!([]);
            } else if !bodies {
                if let Some(exchanges) = finding[1].as_array_mut() {
                    for exchange in exchanges {
                        headers_only(&mut exchange["request"]);
                        headers_only(&mut exchange["response"]);
                    }
                }
            }
        }
    }
    if !data || !initial {
        if let Some(object) = value.as_object_mut() {
            object.remove("initial_request");
            object.remove("initial_response");
        }
    } else if !bodies {
        headers_only(&mut value["initial_request"]);
        headers_only(&mut value["initial_response"]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::audit::RuleResult;

    const RULE: &str = "metadata = {\"id\":\"fixture\",\"type\":\"passive\",\"severity\":\"info\",\"tags\":[\"test\"]}\nfrom cruster import finding, log\ndef scan(request, response):\n    log('fixture log')\n    return [finding('fixture finding', extracted=['ok'])]\n";

    struct Fixture(CrusterMcpServer);
    impl Fixture {
        fn new() -> Self {
            let path = std::env::temp_dir().join(format!(
                "cruster-mcp-audit-test-{:032x}",
                rand::random::<u128>()
            ));
            fs::create_dir(&path).unwrap();
            fs::write(path.join("fixture.py"), RULE).unwrap();
            fs::write(path.join("http.jsonl"), concat!(
                "{\"index\":42,\"request\":{\"method\":\"GET\",\"scheme\":\"https://\",\"host\":\"example.test\",\"path\":\"/path\",\"query\":null,\"version\":\"HTTP/1.1\",\"headers\":[],\"body\":null},\"response\":{\"status\":\"200\",\"version\":\"HTTP/1.1\",\"headers\":[],\"body\":null}}\n"
            )).unwrap();
            let config = AuditConfig {
                passive: true,
                rules: vec![path.join("fixture.py").to_str().unwrap().into()],
                ..Default::default()
            };
            Self(CrusterMcpServer::new(
                path,
                config,
                crate::config::McpConfig {
                    allow_dangerous_tools: false,
                    max_result_bytes: 16384,
                    request_timeout_seconds: 10,
                },
            ))
        }
        fn store_finding(&self) -> String {
            let finding = json!({"id":0,"rule_id":"fixture","about":"fixture","protocol":"http","type":"passive",
                "pair_index":42,"severity":"info","findings":{"test":[["é🦀".repeat(200)],[]]},
                "initial_request":"GET /path HTTP/1.1\r\n\r\nsecret", "initial_response":"HTTP/1.1 200 OK\r\n\r\nsecret"});
            let result: RuleResult = serde_json::from_value(finding).unwrap();
            let text = serde_json::to_string(&result).unwrap();
            fs::write(
                self.0.project.join("audit_saved.jsonl"),
                format!("{text}\n"),
            )
            .unwrap();
            text
        }
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(self.0.project.as_ref());
        }
    }

    fn input<T: serde::de::DeserializeOwned>(value: Value) -> T {
        serde_json::from_value(value).unwrap()
    }

    #[test]
    fn active_options_cannot_bypass_gate_and_paths_are_confined() {
        let fixture = Fixture::new();
        assert!(fixture
            .0
            .audit_config_for(input(json!({"active":true})))
            .is_err());
        assert!(fixture.0.audit_safe_path(Path::new("fixture.py")).is_ok());
        assert!(fixture.0.audit_safe_path(Path::new("/etc/passwd")).is_err());
        let config = fixture
            .0
            .audit_config_for(input(json!({"include":{"tags":["test"]}})))
            .unwrap();
        assert_eq!(load_rules(&config).unwrap().len(), 1);
        let config = fixture
            .0
            .audit_config_for(input(json!({"exclude":{"ids":["fixture"]}})))
            .unwrap();
        assert!(load_rules(&config).unwrap().is_empty());
    }

    #[cfg(unix)]
    #[test]
    fn escaping_symlinks_are_rejected() {
        use std::os::unix::fs::symlink;
        let fixture = Fixture::new();
        symlink("/etc/passwd", fixture.0.project.join("escape.py")).unwrap();
        assert!(fixture.0.audit_safe_path(Path::new("escape.py")).is_err());
        assert!(fixture
            .0
            .audit_config_for(input(json!({"rule_paths":["."]})))
            .is_err());
        symlink("/etc/passwd", fixture.0.project.join("audit_escape.jsonl")).unwrap();
        assert!(fixture.0.audit_result_path("escape").is_err());
    }

    #[test]
    fn pair_selection_and_regex_use_domain_search() {
        let fixture = Fixture::new();
        let selected = fixture
            .0
            .audit_pairs(input(json!({"range":"i42","filters":["example\\.test"]})))
            .unwrap();
        assert_eq!(selected.len(), 1);
        assert_eq!(selected[0].index, 42);
        assert!(fixture
            .0
            .audit_pairs(input(json!({"filters":["absent-value"]})))
            .unwrap()
            .is_empty());
        assert!(fixture
            .0
            .audit_pairs(input(json!({"filters":["["]})))
            .is_err());
        assert!(fixture
            .0
            .audit_pairs(input(json!({"pair_ids":[999]})))
            .is_err());
    }

    #[test]
    fn saved_results_filter_project_and_chunk_losslessly() {
        let fixture = Fixture::new();
        let text = fixture.store_finding();
        let page = fixture.0.audit_results(input(json!({"name":"saved","rule_ids":["fixture"],"pair_ids":[42],"severities":["info"]}))).unwrap().0;
        assert_eq!(page["items"][0]["url"], "https://example.test/path");
        assert!(page["items"][0].get("initial_request").is_none());
        assert_eq!(page["total"], 1);
        let page = fixture
            .0
            .audit_results(input(json!({"name":"saved","rule_ids":["absent"]})))
            .unwrap()
            .0;
        assert_eq!(page["total"], 0);
        let mut offset = 0;
        let mut restored = String::new();
        loop {
            let chunk = fixture
                .0
                .audit_result_content(input(
                    json!({"name":"saved","id":0,"offset":offset,"length":17}),
                ))
                .unwrap()
                .0;
            restored.push_str(chunk["content"].as_str().unwrap());
            if let Some(next) = chunk["next_offset"].as_u64() {
                offset = next as usize;
            } else {
                break;
            }
        }
        assert_eq!(restored, text);
        assert!(fixture
            .0
            .audit_result_content(input(json!({"name":"saved","id":0,"offset":text.len()+1})))
            .is_err());
        assert!(fixture
            .0
            .audit_result_content(input(json!({"name":"saved","id":999})))
            .is_err());
    }

    #[tokio::test]
    async fn inline_and_active_test_rules_require_permission() {
        let fixture = Fixture::new();
        assert!(fixture
            .0
            .test_audit_rule(input(
                json!({"rule":{"source":"inline","python":RULE},"pair":{"source":"saved","id":42}})
            ))
            .await
            .is_err());
        fs::write(
            fixture.0.project.join("active.py"),
            RULE.replace("\"passive\"", "\"active\""),
        )
        .unwrap();
        assert!(fixture.0.test_audit_rule(input(json!({"rule":{"source":"file","path":"active.py"},"pair":{"source":"saved","id":42}}))).await.is_err());
        assert!(fixture
            .0
            .execute_audit(input(json!({"tasks":0})))
            .await
            .is_err());
    }

    #[tokio::test]
    async fn execution_persists_cli_findings_and_refuses_overwrite() {
        let fixture = Fixture::new();
        let result = fixture
            .0
            .execute_audit(input(
                json!({"persist":true,"name":"run","verbose":true,"tasks":1}),
            ))
            .await
            .unwrap()
            .0;
        assert_eq!(result["executions"], 1);
        assert_eq!(result["findings"], 1);
        assert_eq!(result["entries"][0]["pair_id"], 42);
        let path = fixture.0.project.join("audit_run.jsonl");
        let saved = fs::read_to_string(&path).unwrap();
        let finding: RuleResult = serde_json::from_str(saved.trim()).unwrap();
        assert_eq!(finding.get_id(), 0);
        assert_eq!(finding.get_pair_index(), 42);
        assert!(fixture
            .0
            .execute_audit(input(json!({"persist":true,"name":"run"})))
            .await
            .is_err());
        assert_eq!(fs::read_to_string(path).unwrap(), saved);
    }

    #[tokio::test]
    async fn cli_and_mcp_run_share_filtered_execution_and_saved_findings() {
        let fixture = Fixture::new();
        // Include a second pair so this checks execution selection, not merely parsing.
        let history = fixture.0.project.join("http.jsonl");
        let first = fs::read_to_string(&history).unwrap();
        let second = first
            .replace("42", "99")
            .replace("example.test", "excluded.test");
        fs::write(&history, format!("{first}{second}")).unwrap();
        let config = crate::config::Config {
            project: Some(fixture.0.project.to_str().unwrap().to_owned()),
            ..Default::default()
        };
        let mut args = crate::config::parse_cmd_from(
            [
                "cruster",
                "cli",
                "audit",
                "run",
                "cli",
                "--filter",
                "example\\.test",
            ]
            .into_iter()
            .map(Into::into)
            .collect(),
        );
        let (_, command) = args.remove_subcommand().unwrap();
        crate::cli::launch(command, config, fixture.0.audit_config.clone())
            .await
            .unwrap();
        let output = fixture
            .0
            .execute_audit(input(json!({
                "tasks":1, "persist":true, "name":"mcp", "verbose":true,
                "selection":{"filters":["example\\.test"]}
            })))
            .await
            .unwrap()
            .0;
        assert_eq!(output["pairs"], 1);
        assert_eq!(output["executions"], 1);
        assert_eq!(output["findings"], 1);
        assert_eq!(output["entries"][0]["logs"][0]["message"], "fixture log");
        let read = |name| {
            let file = File::open(fixture.0.project.join(format!("audit_{name}.jsonl"))).unwrap();
            crate::audit::operations::query_findings(BufReader::new(file), Default::default())
                .unwrap()
                .map(|r| serde_json::to_value(r.unwrap()).unwrap())
                .collect::<Vec<_>>()
        };
        let cli = read("cli");
        assert_eq!(cli.len(), 1);
        assert_eq!(cli, read("mcp"));
        let page = fixture
            .0
            .audit_results(input(json!({
                "name":"cli", "include_data":true, "include_initial":true, "include_bodies":true
            })))
            .unwrap()
            .0;
        let mut expected = cli[0].clone();
        expected["url"] = json!("https://example.test/path");
        assert_eq!(page["items"][0], expected);
    }

    #[tokio::test]
    async fn mcp_output_budget_does_not_stop_shared_execution_or_persistence() {
        let mut fixture = Fixture::new();
        let history = fixture.0.project.join("http.jsonl");
        let first = fs::read_to_string(&history).unwrap();
        fs::write(&history, format!("{first}{}", first.replace("42", "99"))).unwrap();
        fixture.0.settings.max_result_bytes = 1024;
        let output = fixture
            .0
            .execute_audit(input(json!({
                "tasks":1, "persist":true, "name":"bounded", "verbose":true
            })))
            .await
            .unwrap()
            .0;
        assert_eq!(output["executions"], 2);
        assert_eq!(output["findings"], 2);
        assert_eq!(output["omitted_entries"], 2);
        assert_eq!(output["entries"], json!([]));
        let saved = fs::read_to_string(fixture.0.project.join("audit_bounded.jsonl")).unwrap();
        let results =
            crate::audit::operations::query_findings(saved.as_bytes(), Default::default())
                .unwrap()
                .collect::<std::io::Result<Vec<_>>>()
                .unwrap();
        assert_eq!(
            results.iter().map(RuleResult::get_id).collect::<Vec<_>>(),
            [0, 1]
        );
        assert_eq!(
            results
                .iter()
                .map(RuleResult::get_pair_index)
                .collect::<Vec<_>>(),
            [42, 99]
        );
    }

    #[tokio::test]
    async fn debug_rule_without_pair_matches_cli_all_history_mode() {
        let fixture = Fixture::new();
        let result = fixture
            .0
            .debug_audit_rule(input(
                json!({"rule":{"source":"configured","id":"fixture"},"limit":1}),
            ))
            .await
            .unwrap()
            .0;
        assert_eq!(result["total"], 1);
        assert_eq!(result["items"][0]["pair_id"], 42);
        assert!(result["items"][0].get("state").is_some());
        assert!(result["items"][0].get("logs").is_some());
        assert!(result["next_offset"].is_null());
    }

    #[tokio::test]
    async fn raw_request_uses_origin_form_path_without_network() {
        let fixture = Fixture::new();
        let result = fixture.0.test_audit_rule(input(json!({"rule":{"source":"configured","id":"fixture"},
            "pair":{"source":"raw","url":"https://example.test/base","request":"GET /raw?q=yes HTTP/1.1\r\nHost: example.test\r\n\r\n", "response":"HTTP/1.1 200 OK\r\n\r\n"}}))).await.unwrap().0;
        assert!(result["state"]["Finished"]["initial_request"]
            .as_str()
            .unwrap()
            .contains("/raw?q=yes"));
        assert!(result["logs"].as_array().is_some());
    }

    #[tokio::test]
    async fn inline_rule_file_is_cleaned_up_and_fetch_requires_permission() {
        let mut fixture = Fixture::new();
        let pair = json!({"source":"raw","url":"https://example.test/","request":"GET / HTTP/1.1\n\n","fetch_response":true});
        assert!(fixture
            .0
            .test_audit_rule(input(
                json!({"rule":{"source":"configured","id":"fixture"},"pair":pair})
            ))
            .await
            .is_err());
        fixture.0.settings.allow_dangerous_tools = true;
        let result = fixture
            .0
            .test_audit_rule(input(
                json!({"rule":{"source":"inline","python":RULE},"pair":{"source":"saved","id":42}}),
            ))
            .await
            .unwrap()
            .0;
        assert!(result["state"]["Finished"].is_object());
        assert!(!fs::read_dir(fixture.0.project.as_ref())
            .unwrap()
            .any(|entry| entry
                .unwrap()
                .file_name()
                .to_string_lossy()
                .starts_with(".mcp-audit-")));
        assert!(fixture.0.test_audit_rule(input(json!({"rule":{"source":"inline","python":"invalid metadata"},"pair":{"source":"saved","id":42}}))).await.is_err());
        assert!(!fs::read_dir(fixture.0.project.as_ref())
            .unwrap()
            .any(|entry| entry
                .unwrap()
                .file_name()
                .to_string_lossy()
                .starts_with(".mcp-audit-")));
    }

    #[test]
    fn pagination_adapts_to_bytes_and_sorting_precedes_pagination() {
        let mut fixture = Fixture::new();
        let text = fixture.store_finding();
        let mut one: Value = serde_json::from_str(&text).unwrap();
        one["findings"] = json!({"test":[["ok"],[]]});
        one["severity"] = json!("high");
        let mut two = one.clone();
        two["id"] = json!(1);
        two["severity"] = json!("info");
        fs::write(
            fixture.0.project.join("audit_saved.jsonl"),
            format!("{one}\n{two}\n"),
        )
        .unwrap();
        let first = fixture
            .0
            .audit_results(input(json!({"name":"saved","limit":1,"sort":"severity"})))
            .unwrap()
            .0;
        assert_eq!(first["items"][0]["id"], 1);
        assert_eq!(first["next_offset"], 1);
        let second = fixture
            .0
            .audit_results(input(
                json!({"name":"saved","offset":1,"limit":1,"sort":"severity"}),
            ))
            .unwrap()
            .0;
        assert_eq!(second["items"][0]["id"], 0);
        assert!(second["next_offset"].is_null());
        fixture.0.settings.max_result_bytes = serde_json::to_vec(&first).unwrap().len() + 5;
        let bounded = fixture
            .0
            .audit_results(input(json!({"name":"saved","limit":100,"sort":"severity"})))
            .unwrap()
            .0;
        assert_eq!(bounded["items"].as_array().unwrap().len(), 1);
        assert_eq!(bounded["next_offset"], 1);
    }

    #[test]
    fn raw_response_parser_validates_and_preserves_duplicates() {
        let response =
            parse_response("HTTP/1.1 200 OK\r\nSet-Cookie: a=1\r\nSet-Cookie: b=2\r\n\r\nbody\n")
                .unwrap();
        assert_eq!(response.status, "200");
        assert_eq!(response.headers.get_all("set-cookie").iter().count(), 2);
        assert_eq!(response.body, b"body\n");
        for response in [
            "bad",
            "HTTP/1.1 nope\n\n",
            "HTTP/8 200 OK\n\n",
            "HTTP/1.1 200 OK\nx\n\n",
        ] {
            assert!(parse_response(response).is_err());
        }
    }

    #[test]
    fn names_are_strict_basenames() {
        for name in [
            "",
            ".",
            "..",
            "../other",
            "/tmp/other",
            "a/b",
            "a\\b",
            "a\n",
            "a\0",
        ] {
            assert!(validate_name(name).is_err(), "{name:?}");
        }
        for name in ["scan", "scan-1_test.2", "...test"] {
            validate_name(name).unwrap();
        }
        assert!(validate_name(&"a".repeat(129)).is_err());
    }

    #[test]
    fn ranges_use_positions_or_explicit_ids() {
        let ids = [3, 8, 42, 90];
        for (range, expected) in [
            ("a", vec![3, 8, 42, 90]),
            ("2", vec![3, 8]),
            ("2-3", vec![8, 42]),
            ("-2", vec![42, 90]),
            ("i42", vec![42]),
            ("42i", vec![42]),
            ("0", vec![]),
            ("99-100", vec![]),
        ] {
            assert_eq!(
                select_range(&ids, Some(range)).unwrap(),
                expected.into_iter().collect()
            );
        }
        for range in ["0-1", "3-1", "i1", "x", "-", "1-", "1-2-3"] {
            assert!(select_range(&ids, Some(range)).is_err(), "{range}");
        }
    }

    #[test]
    fn selectors_extend_config_like_cli() {
        let mut entities = Some(AuditEntities {
            ids: vec!["a".into()],
            ..Default::default()
        });
        extend_entities(
            &mut entities,
            AuditEntitiesInput {
                ids: vec!["b".into()],
                tags: vec!["tag".into()],
                ..Default::default()
            },
        );
        let entities = entities.unwrap();
        assert_eq!(entities.ids, ["a", "b"]);
        assert_eq!(entities.tags, ["tag"]);
    }

    #[test]
    fn projection_removes_all_exchange_bodies() {
        let full = json!({"initial_request":"GET / HTTP/1.1\r\n\r\nsecret", "initial_response":"HTTP/1.1 200 OK\n\nsecret", "findings":{"x":[["e"],[{"request":"POST / HTTP/1.1\n\nsecret","response":"HTTP/1.1 200 OK\r\n\r\nsecret","payloads":["x"]}]]}});
        let mut value = full.clone();
        project_result(&mut value, true, true, false);
        assert!(!value.to_string().contains("secret"));
        let mut value = full;
        project_result(&mut value, false, true, true);
        assert!(value.get("initial_request").is_none());
        assert_eq!(value["findings"]["x"][1], json!([]));
        assert_eq!(value["findings"]["x"][0], json!(["e"]));
    }

    #[test]
    fn schemas_are_strict_and_generated() {
        let _ = schemars::schema_for!(AuditExecuteInput);
        let _ = schemars::schema_for!(AuditTestInput);
        let _ = schemars::schema_for!(AuditResultsInput);
        let _ = schemars::schema_for!(AuditResultContentInput);
        assert!(serde_json::from_value::<AuditExecuteInput>(json!({"shell":"anything"})).is_err());
        assert!(serde_json::from_value::<AuditTestInput>(
            json!({"rule":{"source":"inline","python":"x"},"pair":{"source":"saved","id":1}})
        )
        .is_ok());
    }
}
