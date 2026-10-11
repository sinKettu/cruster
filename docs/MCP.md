# Model Context Protocol server

Cruster can run as a local MCP server over standard input/output. This allows an MCP-compatible AI agent to inspect a persisted Cruster project, analyze captured HTTP traffic, run configured Python audit rules, and—with explicit authorization—resend captured requests.

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

MCP starts in read-only/passive mode. The following operations are denied by default:

- sending HTTP requests;
- running active Python audit rules;
- any operation that can change remote state.

They can be enabled explicitly:

```shell
cruster --project my-project --audit-config rules/audit.yaml \
  mcp --allow-dangerous-tools
```

Enabling dangerous tools authorizes the connected agent to send requests to targets represented by the project and active rules. Use it only with trusted agents, rules, and projects. Python rules are extension code, not a security sandbox.

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

Resends the original request from a persisted pair. This tool requires `--allow-dangerous-tools`.

Input fields:

- `id`: HTTP pair ID;
- `max_redirects`: redirect limit, default `5`.

The request is bounded by `--request-timeout-seconds`. The response reports whether the redirect limit was reached.

## Limits

- `--max-result-bytes <BYTES>` controls the maximum serialized tool result. Default: 1 MiB; accepted range: 1 KiB through 16 MiB.
- `--request-timeout-seconds <SECONDS>` controls direct-send timeout. Default: 30 seconds; accepted range: 1 through 600 seconds.
- History list pages are capped at 100 records.
- MCP inputs never accept arbitrary filesystem paths. History is always read from the selected project's `http.jsonl`, and rules come only from the resolved audit configuration.

If a result exceeds the configured limit, request fewer records or omit bodies.

## Current scope

Version 0.8.3 provides persisted-project inspection, Python audit execution, and request replay. MCP does not yet control the lifetime of an already running interactive proxy, mutate captured history, or expose arbitrary file/shell operations.
