# Cruster YAML Config

You can configure `Cruster` either with config file or with command line arguments. I think config is more handy.

You cannot configure path to config (`-c`) and path to workplace (`-w`) with config-file.

## Config

| Key | Value Type | Default | Comment |
| --- | --- | --- | --- |
| tls_key_name | *String* | `~/.cruster/cruster.key` | Path to TLS key in PEM format. This option is not configurable by cmdline and it is required to be written explicitly. |
| tls_cer_name | *String* | `~/.cruster/cruster.cer` | Path to TLS CA certificate in PEM format. This option is not configurable by cmdline and it is required to be written explicitly. |
| address | *String* | `127.0.0.1` | Address for proxy to bind |
| port | *Integer* | `8080` | Port for proxy to listen to |
| debug_file | *String* or `null` | `null` | Path to file to write debug logs. Mostly used for development, for now you will not find there anything useful |
| dump_mode | *JSON* or `null` | `null` | Subconfig to maintain dump mode. See `Dump` section for details. |
| store | *String* or `null` | `null` | Path to store data collected by proxy (requests and responses) in JSONLines format (see `Stored HTTP Data Format.md` for details) |
| load | *String* or `null` | `null` | Path to file with previously stored data by `store` option to load on start |
| scope | *JSON* or `null` | `null` | Subconfig to maintain scope. It allows to include/exclude requests by regexes for URIs. See `Scope` section for details. |

## Dump

| Key | Value Type | Default | Comment |
| --- | --- | --- | --- |
| enabled | *Boolean* | `false` | Toggle dump mode. |
| verbosity | *Integer* | `0` | Verbosity in dump mode, ignored in intercative mode. 0: request/response first line, 1: 0 + response headers, 2: 1 + request headers, 3: 2 + response body, 4: 3 + request body |
| color | *Boolean* | `true` | If `true` Cruster will print colorized lines and black-white otherwise |

## Scope

| Key | Value Type | Default | Comment |
| --- | --- | --- | --- |
| include | *List[String]* or `null` | `null` | List of regular expressions for requests' URIs. Request-Response pair included if matched. |
| exclude | *List[String]* or `null` | `null` | List of regular expressions for requests' URIs. Request-Response pair excluded if matched. Processed after previous list, so you can exclude some sub-matches (see example). |
| strict | *Boolean* | `false` | If `true` data from proxy **will not** be stored at all or skipped on loading from file in case it's not included |

## Example

``` yaml
tls_key_name: /home/user/.cruster/cruster.key
tls_cer_name: /home/user/.cruster/cruster.cer
address: 127.0.0.1
port: 8080
debug_file: ~
dump_mode: false
store: /home/user/.cruster/data.jsonl
load: /home/user/.cruster/saved_data.jsonl
scope:
  strict: false
  include:
    - "^https?://www\\.example\\.com/.*$"
  exclude:
    - "^https?://www\\.example\\.com/exact/unwanted/path/?$"
```
