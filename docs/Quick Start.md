# Cruster quick start

This guide shows the simplest workflow for Cruster:

1. capture traffic in dump mode;
2. inspect or audit it through the CLI;
3. give an AI agent access through MCP.

## Before you start

Build and install Cruster, then make sure `cruster` is available in your shell.

On the first operational launch, Cruster creates:

- the workplace `~/.cruster`;
- the main configuration `~/.cruster/config.yaml`;
- the default project `~/.cruster/default`;
- a local CA certificate and key when the proxy starts.

Commands in this guide use the default project. To use another project, place the global option before the command:

```shell
cruster --project my-project dump
```

Global options such as `--project` and `--audit-config` must appear before `dump`, `cli`, or `mcp`.

## Scenario 1: capture browser or application traffic

Use dump mode when you want Cruster to work as a proxy, print traffic in the terminal, and save it for later analysis.

Start the proxy:

```shell
cruster dump
```

Cruster listens on `127.0.0.1:8080` by default. Configure your browser, command-line client, or application to use this address as its HTTP/HTTPS proxy.

For HTTPS traffic, import `~/.cruster/cruster.cer` as a trusted local CA certificate in the client or browser.

Increase terminal detail with `-v`:

```shell
cruster dump -v
```

More `v` characters show more data:

```shell
cruster dump -vvvv
```

Stop the proxy with `Ctrl+C`. Captured HTTP data is stored in:

```text
~/.cruster/default/http.jsonl
```

### Use another port or project

```shell
cruster --project test-api --port 8081 dump -vv
```

This creates or uses `~/.cruster/test-api` and listens on port `8081`.

### Enable background audit while capturing

The global `-A` option is intended for background audit in dump or interactive mode:

```shell
cruster -A --audit-config /path/to/audit.yaml dump -v
```

For explicit CLI or MCP audit commands, `-A` is not required.

## Scenario 2: inspect captured traffic through the CLI

Use CLI mode after dump mode has saved traffic. The default project is selected automatically.

Show a compact table with all stored exchanges:

```shell
cruster cli http show a
```

Show the first ten exchanges:

```shell
cruster cli http show 10
```

Show full requests and responses:

```shell
cruster cli http show a --pretty
```

Show only IDs and URLs:

```shell
cruster cli http show a --urls
```

Filter output with a regular expression:

```shell
cruster cli http show a --filter 'google|example'
```

For command-specific options, append `--help`:

```shell
cruster cli http show --help
```

Running `cruster` without arguments is treated as `cruster cli` and prints the list of required CLI subcommands.

### Run Python audit rules

Specify an audit configuration and run passive rules:

```shell
cruster \
  --audit-config /path/to/audit.yaml \
  cli audit run quick-check --passive --verbose
```

Run both passive and active rules only when network requests are expected and authorized:

```shell
cruster \
  --audit-config /path/to/audit.yaml \
  cli audit run full-check --passive --active --verbose
```

`-A` is not required in CLI mode. The audit command itself explicitly requests execution.

If the audit configuration does not contain rule paths, provide one directly:

```shell
cruster cli audit run quick-check \
  --passive \
  --rule-path /path/to/python/rules
```

### Work with a different project

```shell
cruster --project test-api cli http show a
```

The global `--project` option goes before `cli`.

## Scenario 3: let an AI agent inspect the project through MCP

Use MCP mode when an MCP-compatible agent should inspect history or run audit rules. Cruster communicates over standard input/output; it does not open an MCP network port.

Start an MCP server for history access:

```shell
cruster mcp
```

Expose configured Python audit rules as well:

```shell
cruster --audit-config /path/to/audit.yaml mcp
```

Normally the MCP client starts this command itself. Example client configuration:

```json
{
  "mcpServers": {
    "cruster": {
      "command": "/path/to/cruster",
      "args": [
        "--audit-config",
        "/path/to/audit.yaml",
        "mcp"
      ]
    }
  }
}
```

After connecting, ask the agent things such as:

- "Show the status of the Cruster project.";
- "List the first ten captured HTTP exchanges without bodies.";
- "Show HTTP pair 3.";
- "List available audit rules.";
- "Run passive audit rules for pairs 0 through 5 and explain the findings.".

### Allow request sending and active rules

MCP is safe by default: it can read the project and run passive rules, but it cannot send requests or run active rules.

Enable those capabilities explicitly only for a trusted agent and target:

```shell
cruster --audit-config /path/to/audit.yaml \
  mcp --allow-dangerous-tools
```

The agent can then resend stored requests and execute active Python rules that may generate network traffic.

`-A` is not required in MCP mode. MCP tool calls explicitly request audit execution.

## Typical workflow

### Capture, inspect, then ask an agent

1. Capture traffic:

   ```shell
   cruster dump -v
   ```

2. Stop it with `Ctrl+C` and inspect the history:

   ```shell
   cruster cli http show a --urls
   ```

3. Configure your MCP client to start:

   ```shell
   cruster --audit-config /path/to/audit.yaml mcp
   ```

4. Ask the agent to inspect selected pairs and run passive rules.

## Where to read more

- Detailed CLI commands: [CLI.md](CLI.md)
- MCP contracts and security model: [MCP.md](MCP.md)
- Python audit rule API: [Python Audit Rules.md](Python%20Audit%20Rules.md)
- Main YAML configuration: [Cruster YAML Config Format.md](Cruster%20YAML%20Config%20Format.md)
- Stored HTTP format: [Stored HTTP Data Format.md](Stored%20HTTP%20Data%20Format.md)
