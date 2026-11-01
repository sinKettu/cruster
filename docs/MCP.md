# Model Context Protocol server

Cruster can run as a local MCP server over standard input/output. This allows an MCP-compatible AI agent to inspect a persisted Cruster project, analyze captured HTTP traffic, run configured Python audit rules, and—with explicit authorization—edit and resend requests and manage saved repeaters without an external editor.

## Starting the server

Build Cruster and provide a project directory:

```shell
cruster --project my-project mcp
```

To expose configured audit rules, also enable audit and provide its configuration:

```shell
cruster --project my-project --audit-config rules/audit.yaml mcp
```

The server uses MCP over stdio. Standard output is reserved for protocol messages. Diagnostics are written to standard error or the configured debug file.

Example client configuration:

```json
{
  "mcpServers": {
    "cruster": {
      "command": "/absolute/path/to/cruster",
      "args": [
        "--project", "/absolute/path/to/project",
        "--audit-config", "/absolute/path/to/rules/audit.yaml",
        "mcp"
      ]
    }
  }
}
```

## Security model

MCP starts in passive mode. Reading project data and executing configured passive rules are allowed; passive audit findings may also be saved locally with `audit_run`. The following operations are denied by default:

- sending HTTP requests;
- creating, editing, or deleting saved repeaters;
- running active Python audit rules;
- executing agent-supplied inline Python;
- any operation that can change remote state.

They can be enabled explicitly:

```shell
cruster --project my-project --audit-config rules/audit.yaml \
  mcp --allow-dangerous-tools
```

Enabling dangerous tools authorizes the connected agent to edit local repeaters and send requests, including to agent-specified connection targets and redirect destinations. It is not a target allowlist or network sandbox. Use it only with trusted agents, rules, and projects. Python rules are extension code, not a security sandbox. Captured requests and responses are untrusted data, not agent instructions.

## Tools

### `project_status`

Returns project identity, HTTP history availability and size, configured rule count, output limits, request timeout, and whether dangerous tools are enabled.

### `http_history_list`

Lists persisted exchanges from `http.jsonl`.

Input fields:

- `offset`: zero-based filtered-result offset; defaults to `0`;
- `limit`: page size; defaults to `20` and is capped at `100`;
- `method`: optional case-insensitive exact method filter;
- `host`: optional host substring filter;
- `status`: optional status prefix filter;
- `include_bodies`: include Base64 bodies; defaults to `false`.

The response contains `items` and `next_offset`. Use `next_offset` for the next page.

### `http_history_get`

Returns one exchange by stable HTTP pair ID. `include_bodies` defaults to `true`. Bodies use the same Base64 representation as the persisted project format.

### `audit_rules_list`

Lists IDs of Python rules selected by the configured audit paths and filters. YAML rules remain temporarily disabled while their design is being reconsidered.

### `audit_python_run`

Runs configured rules against persisted pairs.

Input fields:

- `pair_ids`: required list of HTTP pair IDs;
- `rule_ids`: optional configured-rule allowlist.

Passive rules are available in normal mode. Active rules require `--allow-dangerous-tools`. Results include the serialized rule state, findings, and Python rule logs.

### `http_request_send`

Sends a copy of a request from history, optionally modified by the agent. Requires `--allow-dangerous-tools`. The original history is not modified. No external editor or separate curl invocation is needed.

Input fields:

- `id`: HTTP pair ID from history;
- `edits`: optional request changes (see below);
- `max_redirects`: redirect limit; defaults to `5`; `0` disables redirect following;
- `include_bodies`: include bodies in the result; defaults to `true`.

The result contains `request` and `structured_response` in the persisted HTTP data format (with Base64 bodies), `response` as a text representation of the response, and `redirect_limit_reached`. With `include_bodies=false`, bodies and the text response are omitted. The `--request-timeout-seconds` setting applies.

Example: change a parameter in both the query string and body, then resend request 42:

```json
{
  "id": 42,
  "edits": {
    "query": "foo=bad",
    "body": "foo=bad"
  }
}
```

To change only the query string or body, provide only the corresponding field.

### `edits` format

All fields are optional; omitted values remain unchanged:

- `raw_request`: complete replacement of the request text, including the request line, headers, blank line, and body. The request line uses origin-form: `POST /path?query HTTP/1.1`. The connection address comes from the original request or `url`;
- `method`, `version`: HTTP method and version;
- `url`: absolute HTTP(S) connection URL; usernames, passwords, and fragments are not allowed;
- `path`: path without the query string;
- `query`: complete query string without `?`; an empty string removes the query;
- `headers`: complete replacement of the headers with an array of `{"name":"…","value":"…"}` objects. Duplicate names are preserved; an empty array removes all headers;
- `remove_headers`: list of header names to remove, matched case-insensitively;
- `set_headers`: replace all values of the listed headers; duplicate names in the array are preserved;
- `body`: exact UTF-8 body, including blank lines and trailing spaces; an empty string clears the body;
- `body_base64`: exact binary body encoded in Base64; cannot be supplied together with `body`.

Changes are applied in this order: `raw_request`, then `url`, `path`, `query`, complete header replacement, header removal, header setting, and body. The other structured fields also override values from `raw_request`. Unknown fields are rejected so that a typo cannot silently cause the original request to be sent.

The connection address and Host header are independent: changing `url` does not replace Host. To change both, also provide `set_headers`. The transport calculates Content-Length from the final body. HTTP/2 uses Cruster's shared HTTP sender, including Host-to-`:authority` conversion and filtering of incompatible headers.

### Saved repeaters

MCP uses the same `repeater.jsonl` file as the CLI and TUI. Requests are returned to the agent as plain text; the server handles the file's Base64 representation. All write and send operations require `--allow-dangerous-tools`.

A repeater ID is a **one-based position**, as in the CLI, not a history pair ID. Deleting a repeater shifts subsequent IDs; refresh the list before taking further action. HTTP history is not modified.

#### `repeater_list`

Input fields: `offset` (defaults to `0`) and `limit` (defaults to `20`, maximum `100`). Returns `items` with each repeater's ID, name, parameters, and an indicator of whether a response is saved, along with `next_offset`. Bodies are omitted.

#### `repeater_get`

Input fields: `id` or `name`, and `include_bodies` (defaults to `true`). Returns `name`, the request text in `request`, the latest response text in `response`, and `parameters`: `address`, `https`, `redirects`, and `max_redirects`. With `include_bodies=false`, only the request and response headers are returned.

#### `repeater_create`

Creates a repeater from `history_id` or `raw_request`. For a new request without a history entry, `changes.address` and `changes.https` are required. `raw_request` can also replace a request copied from history. The optional `changes` object lets you set the name, request, and parameters during creation. Returns the created repeater without sending the request.

```json
{
  "history_id": 42,
  "changes": {
    "name": "Test foo",
    "edits": {"query": "foo=bad", "body": "foo=bad"}
  }
}
```

#### `repeater_update`

Input fields: `id` and `changes`. The `changes` object accepts:

- `name`;
- `edits` in the format described above;
- `address`: connection hostname or IP address with an optional port; IPv6 addresses must be enclosed in square brackets;
- `https`: whether to use TLS;
- `redirects`: whether to follow redirects;
- `max_redirects`: redirect limit.

Omitted parameters remain unchanged. Address settings are applied before `edits`, so an explicitly supplied `edits.url` takes precedence. Updating a repeater clears its previous response but does not send the request.

#### `repeater_send`

Input fields: `id`, optional `changes`, and `include_bodies` (defaults to `true`). Applies and saves changes, sends the request, and saves the latest response. Returns the same data as `http_request_send`, plus `repeater_id`.

```json
{
  "id": 1,
  "changes": {"edits": {"body": "foo=bad"}},
  "include_bodies": false
}
```

Changes are saved before the network call; the response is saved before the result size limit is checked. If a network error or timeout occurs, the request may already have reached the server: do not automatically retry a state-changing operation. If the result is too large, read the saved response rather than sending the request again.

#### `repeater_content_get`

Reads a saved request or response in chunks without sending a network request. Input fields: `id`, `part` (`request` or `response`), `offset` (byte offset, defaults to `0`), and `limit` (defaults to 16384 bytes, maximum 65536, automatically reduced to fit the result size limit).

Returns `data_base64` with the exact chunk bytes, `text` (if the entire chunk is valid UTF-8), `total_bytes`, `offset`, and `next_offset`. Continue reading from `next_offset` until it becomes `null`. This lets the agent analyze large responses without opening files manually.

#### `repeater_delete`

Selection fields: `id` **or** `name`. Deletes the repeater and returns `deleted_id` and the `remaining` count. Subsequent IDs are renumbered.

All tools that select a saved repeater (`repeater_get`, `repeater_update`, `repeater_send`, `repeater_delete`, and `repeater_content_get`) accept an exact `name` instead of `id`. Provide exactly one selection field. If names are duplicated, the first matching repeater is selected, as in the CLI. Use `changes.name` to rename a repeater.

With `repeater_get` and `include_bodies=false`, **both request and response** headers are retained, but bodies are omitted.

## CLI command and option mapping

MCP provides the same data operations without launching the CLI as a child process. Parameter names and representations are adapted to structured tool calls.

### Shared CLI and MCP implementation

MCP uses CLI operations directly or shared code extracted from existing CLI handlers:

- [HTTP query](../src/cli/http/show.rs) — shared range parsing, selection by position or ID, extraction, and regular expression filtering. Range parsing is also used to select pairs for audits.
- [History reading](../src/cli/http.rs) — shared JSONL line handling for the CLI and MCP, including incomplete records in follow mode.
- [Repeater operations](../src/cli/repeater/operations.rs) — shared selection by number or name, creation from history, and the save → send → save response cycle. [Reading and atomic writing](../src/cli/repeater.rs) use the same CLI/TUI storage format, without a separate MCP file model.
- [Audit execution](../src/audit/execution.rs) — shared execution using existing CLI workers, including single-rule execution. [Audit operations](../src/audit/operations.rs) combine pair filtering with reading and sorting findings.

Only interface-specific behavior remains separate: the CLI prints results and opens an editor when needed; MCP checks permissions, returns JSON, limits response size, and supports pagination and cursors. Changes to shared selection, execution, or persistence logic apply to both interfaces.

| CLI | MCP | Options and behavior |
| --- | --- | --- |
| `http show` | `http_history_query` | All INDEX ranges, exact ID, regular expressions, extraction, URLs, summary, raw JSONL, pretty output, and body omission |
| `http follow` | `http_history_follow` | New records with a cursor, `no_old_lines`, and URLs; bounded waiting instead of continuous output |
| `repeater list` | `repeater_list` | Names, parameters, and pagination |
| `repeater show` | `repeater_get` | Number or name; headers without bodies |
| `repeater add` | `repeater_create` | Captured pair ID; also supports creating a new request |
| `repeater edit` | `repeater_update` | Name, address, HTTPS, redirects, and max_redirects; also supports editing the request itself |
| `repeater exec` | `repeater_send` | Number or name, changes before sending, and body omission. No external editor is needed |
| `audit run` | `audit_run` | Name and persistence, HTTP regular expression filters, rule paths, inclusion/exclusion by ID and tags, active/passive selection, verbosity, and concurrency |
| `audit test` | `audit_run` without persistence | Test configured rules against selected history. The unused CLI argument ARG is not exposed |
| Single-rule testing | `audit_test` | Configured rule, file, or inline Python; saved pair or explicitly supplied request/response |
| `audit debug-rule` | `audit_debug_rule` | Rule, specific pair or entire history; states and Python logs instead of terminal debug output |
| `audit print` | `audit_results_list` | One or all findings, sorting by severity/rule ID, initial data, sent request/response data, and body/data omission |
| `help`, `--help` | MCP `tools/list` | Tool descriptions and typed parameter schemas |

Only terminal-specific features are omitted: colors/ANSI, table widths, an external editor, global console logger settings, continuous output, and short aliases for existing commands. The `--force` option is unnecessary because MCP never opens an editor. Data shown by the CLI in these modes is available in structured form.

Global project, workplace, main configuration, and audit configuration options are set when starting MCP through Cruster. Proxy listener, TLS certificate, and interactive TUI options are not CLI operations on a persisted project and do not provide control over a running proxy.

### `http_history_query`

- `range`: CLI INDEX syntax — `n` (first n records, defaults to `100`), `a` (all), `n-m` (one-based positions, inclusive), `-n` (last n records), or `iN`/`Ni` (pair ID).
- `id`: exact ID, regardless of the range. As in the CLI, an exact ID enables pretty output; use the existing `http_history_get` tool for a structured pair.
- `format`: `summary` (default), `urls`, `structured`, `raw`, or `pretty`. Pretty output contains no ANSI sequences; raw output returns the original JSONL lines.
- `filter`: regular expression applied to request/response first lines, headers, and bodies through the shared HTTP matcher. In raw mode, the expression is applied to the original JSONL line, as in the CLI.
- `extract`: `method=…`, `status=…`, `host=…`, or `path=…`, with the same prefix-matching semantics as the CLI.
- `include_bodies` (defaults to `true`), `no_body` (defaults to `false`). Raw mode is incompatible with extraction and body omission.
- `offset`, `limit`: pagination of the selected result (page size defaults to 20, maximum 100); `content_offset` continues a single large record.

The result contains `items`, `next_offset`, and `next_content_offset`. If a single record exceeds the size limit, it is returned as a Base64 chunk of its JSON encoding, with `encoding`, `media_type`, `data`, `offset`, and `total_bytes` fields. Pass both returned offsets while keeping the other parameters unchanged. Assemble the bytes of a large record before decoding the JSON; this preserves both Unicode and binary bodies in the structured format.

```json
{
  "range": "-50",
  "format": "urls",
  "extract": "status=4",
  "filter": "(?i)token|authorization",
  "limit": 20
}
```

### `http_history_follow`

Input fields: `cursor` (a byte cursor, not a pair ID), `no_old_lines`, `urls`, `wait_ms` (defaults to 1000, maximum 30000), `limit` (defaults to 20, maximum 100), and `content_offset`.

The first call without a cursor reads history from the beginning; `no_old_lines=true` starts after the last complete record. The response contains `items`, `next_cursor`, `next_content_offset`, and `has_more`. To continue, pass `cursor=next_cursor` and `content_offset=next_content_offset`. An incomplete final JSONL line is retained until it is completed. Use a cursor only with the same project and a history file that has not been truncated.

### `http_history_content_get`

Input fields: `id`, `part` (`request`, `response`, or `record`), `offset` in bytes, and `length`. For request/response, returns the exact decoded **body bytes**; for record, returns the original JSONL record. The response contains Base64 `data`, `total_bytes`, and `next_offset`. The chunk size is automatically adjusted to fit the result size limit.

### `audit_run`

- `selection.pair_ids`: HTTP pair IDs; `selection.range`: range; `selection.filters`: additional regular expressions. ID selection is intersected with the range and filters. Multiple expressions are combined with OR and appended to the audit configuration filters.
- `options.passive`, `options.active`: override rule type selection. Active rules require dangerous tools to be enabled.
- `options.include`, `options.exclude`: objects with `ids`, `tags`, and `paths` arrays (`paths` contains regular expressions matched against rule paths). Values extend the configured selectors; `options.reset_selectors=true` clears them first.
- `options.rule_paths`: replace rule loading paths; `options.add_paths`: add paths. Access is allowed within the selected project and configured rule roots; paths outside them, including through symbolic links, are rejected. To use another directory, include it in the MCP startup configuration or supply an inline rule for testing.
- `tasks`: concurrency from 1 to 256; defaults to the audit configuration value or CPU count.
- `verbose`: include execution states and Python logs; defaults to the configuration value.
- `name`: run name; defaults to the configuration value. Must be a safe basename without `audit_` or `.jsonl`.
- `persist`: save findings to the CLI-compatible `audit_NAME.jsonl` file; defaults to `false`. Use `true` for the equivalent of CLI run and `false` for CLI test.

The result contains counts of rules, pairs, executions, findings, errors, and skips, an `entries` array, and an `omitted_entries` count. Details that exceed the size limit are omitted, but persisted findings remain available through the reading tools. Existing result files are not overwritten. An interrupted run may leave a partial file containing findings already written.

```json
{
  "name": "selected",
  "persist": true,
  "selection": {"filters": ["^GET https?://example\\.com/"]},
  "options": {
    "passive": true,
    "active": false,
    "include": {"tags": ["technologies"]}
  },
  "tasks": 4,
  "verbose": true
}
```

### `audit_test` and `audit_debug_rule`

The `rule` field selects a rule:

- `{"source":"configured","id":"rule-id"}`;
- `{"source":"file","path":"rule.py"}` within the project or configured rule roots;
- `{"source":"inline","python":"…"}` — code with standard Cruster Python rule metadata. Requires dangerous tools to be enabled, regardless of the rule type.

The `pair` field accepts:

- `{"source":"saved","id":42}`;
- `{"source":"raw","request":"GET / HTTP/1.1\r\nHost: example.test\r\n\r\n","url":"https://example.test/","response":"HTTP/1.1 200 OK\r\n\r\nbody"}`.

In raw mode, you can explicitly supply `fetch_response=true` instead of `response`: this sends the request and requires dangerous tools to be enabled. No network requests are made implicitly.

Returns `pair_id`, `rule_id`, `state`, and `logs`. For `audit_debug_rule`, the `pair` field is optional: without it, the rule is tested against all saved pairs using `offset`/`limit` pagination; the response contains `items`, `total`, `next_offset`, and `output_truncated`. Requesting the same page again reruns the rule, so do not automatically repeat completed pages for active rules.

The direct HTTP send timeout applies to `fetch_response`, but does not limit the execution time of arbitrary Python code: the current shared Python executor does not support safe forced cancellation. Run only trusted rules.

### `audit_results_list`

Requires `name`; optional fields are finding `id`, `pair_ids`, `rule_ids`, `severities`, and regular expression `filter`. The `sort` field accepts `file` (default), `rule_id`, `severity`, or `severity_rule_id`. Sorting is applied before `offset`/`limit` pagination.

`include_data` includes the finding's sent request/response data, `include_initial` includes the original pair, and `include_bodies` includes bodies. All three default to `false`. These fields provide equivalents of the CLI no-data, initial-data, and without-body options without depending on output formatting. Each finding includes the original pair's URL. Returns `items`, `total`, and `next_offset`.

### `audit_result_content_get`

Input fields: `name`, finding `id`, `offset`, and `length`. Reads a complete finding as JSON regardless of its size, returning UTF-8 `content`, `next_offset`, and the size. Offsets are measured in bytes; chunk boundaries do not split Unicode characters.

### Repeater limitations

- Saved CLI/TUI repeaters support UTF-8 request text and headers. For binary bodies, use `http_request_send` with `body_base64`; binary responses are available in `structured_response.body`.
- `raw_request` is a structured HTTP request, not an arbitrary sequence of TCP bytes: the transport may normalize URLs, headers, and framing. This interface does not provide exact reproduction of malformed HTTP framing, HTTP/3, WebSocket sessions, or CONNECT tunnels.
- MCP serializes access to repeaters within the process and replaces the file atomically. Concurrent writes to the same project from another MCP process, the CLI, or the TUI are not coordinated; use a single writer.

## Limits

- `--max-result-bytes <BYTES>` controls the maximum serialized tool result. Default: 1 MiB; accepted range: 1 KiB through 16 MiB.
- `--request-timeout-seconds <SECONDS>` controls direct-send timeout. Default: 30 seconds; accepted range: 1 through 600 seconds.
- History list pages are capped at 100 records.
- History and repeaters use only the selected project's `http.jsonl` and `repeater.jsonl`. Audit rule paths are confined to the project and configured rule roots; result names are validated basenames.

If a result exceeds the configured limit, request fewer records or omit bodies.

## Current scope

MCP supports project inspection, Python audits, request editing and resending, full lifecycle management of saved repeaters, and chunked reading of large results. It does not provide lifecycle control over an already running interactive proxy, modification of captured history, or arbitrary file operations or command execution.
