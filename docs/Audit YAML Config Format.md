# Cruster Audit YAML Config

You can configure `Cruster` audit with a separate config file. It defines which rules to load and which HTTP Request-Response pairs to check.

You can set path to audit config with `--audit-config` or with `audit_config` in the [main config](Cruster%20YAML%20Config%20Format.md). The file is not loaded automatically just because it is named `audit.yaml`.

Audit config uses YAML. It is not the same thing as a YAML audit rule: YAML rules are temporarily disabled, only Python rules are loaded. See [Python Audit Rules](Python%20Audit%20Rules.md) for rule format and execution details.

## Config

| Key | Value Type | Default | Comment |
| --- | --- | --- | --- |
| active | *Boolean* | Required | Toggle loading of active rules. Active rules can send HTTP requests and change remote state. |
| passive | *Boolean* | Required | Toggle loading of passive rules. Passive rules inspect stored traffic without sending requests. |
| rules | *List[String]* | Required | Paths to existing rule files or directories. Directories are searched recursively. Relative paths are checked in the current working directory first, then inside the Cruster workplace, not relative to the audit config file. |
| tasks | *Integer* or `null` | `null` | Number of audit workers for CLI audit run. If `null` or omitted, Cruster uses the number of logical CPUs. Must be greater than `0`. Workers take rules from a shared queue; each rule processes the selected HTTP pairs. |
| include | *JSON* or `null` | `null` | Subconfig to include rules by paths, IDs and tags. See `Include` section for details. |
| exclude | *JSON* or `null` | `null` | Subconfig to exclude rules by paths, IDs and tags. Processed after corresponding include checks. See `Exclude` section for details. |
| filter | *List[String]* or `null` | `null` | Regular expressions to select HTTP pairs for audit. This option does not select rule files. See `Filter` section for details. |
| verbose | *Boolean* or `null` | `null` | Toggle additional CLI audit output: scan plan, skipped pairs and worker/rule logs. If `null` or omitted, verbose output is disabled. Failures are printed regardless of this option. |
| name | *String* or `null` | `null` | Audit name used in results filename. CLI audit run requires a positional name and uses it instead of this value. |

`active`, `passive` and `rules` must be written explicitly. Other keys can be omitted or set to `null`. At least one rule type must be enabled to load rules. An empty `rules` list loads no rules.

If no audit config is selected, Cruster uses an empty configuration: both rule types are disabled, rule paths are empty and `tasks` is `1`. These values are not substituted for required keys in a YAML file. The bundled `rules/audit.yaml` enables passive rules and loads `rules/example` and `rules/http`, but you still have to select this file explicitly.

Only load trusted rules. Enabling `active` allows real network requests; `filter` is not a restriction on the destinations an active rule can contact.

The audit config path itself is resolved in the same order: current working directory, then workplace. Absolute paths are used as written. A missing configured path is an error while loading the config.

Cruster discovers files ending with `.py`, `.yaml` or `.yml`. Path include/exclude checks are applied before reading rule metadata. YAML rule files are skipped with a warning, and invalid Python rules are ignored with a warning.

Path regexes match the discovered file path, not an HTTP URI or the contents of a rule. A path can be relative or absolute depending on how its search directory was resolved. For example, `(^|/)http/passive/.*\.py$` matches files inside an `http/passive` directory in either case.

## Include

| Key | Value Type | Default | Comment |
| --- | --- | --- | --- |
| tags | *List[String]* | Required | Include a rule if its metadata contains at least one of these tags. An empty list does not restrict tags. |
| paths | *List[String]* | Required | Include a rule file if its path matches at least one of these regular expressions. Does not add search directories. An empty list does not restrict paths. |
| ids | *List[String]* | Required | Include a rule if its ID equals at least one value from this list. An empty list does not restrict IDs. |

If `include` is omitted or `null`, there are no include restrictions. If it is a subconfig, all three lists must be written explicitly, even if some are empty.

Values inside one list are alternatives. Different non-empty lists must all match: a rule must match an included path **and** an included ID **and** at least one included tag. An include match does not enable a disabled rule type and does not override exclusions.

IDs and tags are exact, case-sensitive strings, not regular expressions.

## Exclude

| Key | Value Type | Default | Comment |
| --- | --- | --- | --- |
| tags | *List[String]* | Required | Exclude a rule if its metadata contains at least one of these tags. An empty list excludes nothing by tag. |
| paths | *List[String]* | Required | Exclude a rule file if its path matches at least one of these regular expressions. An empty list excludes nothing by path. |
| ids | *List[String]* | Required | Exclude a rule if its ID equals at least one value from this list. An empty list excludes nothing by ID. |

If `exclude` is omitted or `null`, there are no exclude restrictions. If it is a subconfig, all three lists must be written explicitly, even if some are empty.

A match in any exclude list is enough to skip a rule. Exclusions take priority over includes: you can include a directory or a tag and then exclude particular files or rule IDs. IDs and tags are exact, case-sensitive strings.

## Filter

`filter` selects stored HTTP pairs, not rules. If omitted, `null` or an empty list, all loaded pairs are selected.

Each regular expression is checked against request and response first lines, individual header lines and bodies. A pair is selected if at least one expression matches at least one of these parts. Request and response do not both have to match. The first lines contain method, URI and HTTP version for a request, and HTTP version and status for a response. Headers are checked as `name: value` lines; bodies and header values are decoded to text with replacement of invalid UTF-8 bytes.

These expressions do not search raw JSONLines or Base64-encoded body strings from the [stored HTTP data format](Stored%20HTTP%20Data%20Format.md). Message parts are checked separately, so one match cannot span a first line, headers and body. Expressions are case-sensitive unless regex flags change it. Invalid regular expressions in `filter` or in the `paths` lists of the `include` and `exclude` subconfigs cause an error.

Rule selection and HTTP pair selection are independent. Every loaded rule is scheduled for the selected pairs; Python rules skip pairs without a request or a response. Filtering does not delete records from HTTP history and is not the same thing as the main config's URI scope.

## Example

``` yaml
active: false
passive: true
rules:
  - rules/example
  - rules/http
tasks: 2
include:
  tags:
    - headers
  paths:
    - '(^|/)http/passive/.*\.py$'
  ids: []
exclude:
  tags:
    - experimental
  paths:
    - '(^|/)deprecated/'
  ids: []
filter:
  - '^GET https?://www\.example\.com/'
verbose: true
name: null
```

This config loads passive rules tagged `headers` from the configured directories, but only if their paths match the include expression and none of the exclude conditions match. The HTTP filter selects pairs whose request first line matches a `GET` request to `www.example.com`.

To run it from the project source directory with the current project's stored HTTP history:

``` shell
cruster --audit-config rules/audit.yaml audit run headers-check
```

Save the example as `rules/audit.yaml` or replace the config path in the command. The positional audit name replaces the configured `name`. Findings are saved as `audit_headers-check.jsonl` inside the current project; running the same name again truncates the previous results file. See [CLI](CLI.md) for command-specific arguments.
