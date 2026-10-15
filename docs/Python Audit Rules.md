# Python audit rules

Cruster executes Python rules inside the RustPython interpreter embedded in the binary. Python is not installed or spawned on the host.

Every rule is a `.py` file that starts at byte zero with a JSON-compatible `metadata` dictionary assignment and provides a `scan(request, response)` entry point. `metadata` contains `id`, `about`, `authors`, `references`, `tags`, `severity` (`info`, `low`, `medium`, or `high`) and `type` (`passive` or `active`). Cruster reads this leading object directly without starting RustPython. Files with comments, imports, blank lines, a byte-order mark, or any other content before `metadata = ` are ignored during rule discovery.

Rule API symbols use a regular import, for example `from cruster import HttpRequest, HttpResponse, finding`. Active rules additionally import `send` and `exchange`. Cruster creates this module inside RustPython before loading a rule. Editor declarations are provided by `python/cruster/__init__.pyi`, while `pyrightconfig.json` makes Pylance and Pyright discover them without ignored diagnostics or editor-specific exceptions. The declarations provide completion, signatures, documentation and request/response type checking.

Requests expose `method`, `uri`, `version`, `headers`, and `body`; responses expose `status`, `version`, `headers`, and `body`. Headers are lists of `{name, value}` dictionaries and values are Base64 strings, preserving duplicate and binary headers. Bodies are Base64 strings. A request has `clone()` and both message types have `to_dict()`.

Return a list created with `finding(name, extracted=[], exchanges=[])`. Returning `[]` means the rule found nothing.

Active rules additionally call `send(request, max_redirects=5)`. This native bridge validates the modified request in Rust, sends it with Cruster's HTTP sender, and returns an `HttpResponse`. Evidence can be attached with `exchange(request, response, payloads=[])`.

Rules are discovered recursively from configured rule paths. Include/exclude IDs, tags, paths, and active/passive switches apply to Python rules. YAML rules and their implementation remain part of the project, but their execution is temporarily disabled while the YAML rule design is being reconsidered: the loader prints a warning and skips them, while direct execution is guarded by an explicit `todo!()`.

The embedded interpreter is an extension boundary, not a security sandbox. Only load trusted rules. Active rules perform real network requests and must be enabled explicitly.

## Python API contract

The public API described below is the complete contract currently exposed by Cruster to Python rules. Import public names from `cruster`; names starting with `__cruster_` are implementation details and are not part of the contract.

```python
from cruster import (
    HttpRequest,
    HttpResponse,
    Exchange,
    Finding,
    exchange,
    finding,
    log,
    send,
)
```

The canonical editor-readable declarations are stored in `python/cruster/__init__.pyi`.

### Rule module contract

Each rule file must define two module-level objects. The exact text `metadata = ` must be the first bytes of the file, and its value must use JSON syntax (including double-quoted keys and strings). Imports must follow the metadata object:

```python
metadata = {
    "id": "unique-rule-id",
    "about": "Human-readable rule description",
    "authors": ["Author Name"],
    "references": ["https://example.org/reference"],
    "tags": ["category", "technology"],
    "severity": "medium",
    "type": "passive"
}

from cruster import HttpRequest, HttpResponse


def scan(request: HttpRequest, response: HttpResponse):
    return []
```

`metadata` is parsed statically when the rule is loaded; loading metadata does not compile or execute Python. Dynamic Python expressions, single-quoted strings, trailing commas, comments inside the object, `True`, `False`, and `None` are not valid in this block. Rules that do not follow this format are ignored with a warning. Its fields have the following contracts:

| Field | Type | Required | Contract |
| --- | --- | --- | --- |
| `id` | `str` | yes | Stable rule identifier used by include/exclude filters and audit output. It should be unique across the loaded rule set. |
| `about` | `str` | no | Human-readable description written to results. Defaults to an empty string. |
| `authors` | `list[str]` | no | Rule authors. Defaults to an empty list. |
| `references` | `list[str]` | no | Related documentation or vulnerability references. Defaults to an empty list. |
| `tags` | `list[str]` | no | Values used by tag filters. Defaults to an empty list. |
| `severity` | `str` | yes | One of `info`, `low`, `medium`, or `high`. |
| `type` | `str` | yes | Either `passive` or `active`. |

`scan(request, response)` is called once for every complete stored HTTP pair selected for the rule. Both arguments represent the original pair. The function must return a list of findings. Returning an empty list means that no issue was found. Returning `None`, a scalar value, or dictionaries that do not match the finding contract causes rule execution to fail with an invalid-findings error.

Cruster batches all selected pairs for one rule. The RustPython interpreter, bootstrap, scan expression, and rule source are compiled once per batch, but every call to `scan()` receives a newly created module scope. The bootstrap and rule module bytecode are executed again in that fresh scope before each scan. Therefore mutable module globals remain isolated between HTTP pairs while Python parsing and bytecode compilation are not repeated for every pair.

Python exceptions and invalid findings are rule failures and are not retried; processing continues with the remaining pairs and rules. If the blocking Python runtime task itself panics or is cancelled, Cruster retries the complete current rule once in a newly created runtime. The per-rule audit task also has the same one-retry boundary. If either task crashes again, the rule is reported as failed for its selected pairs and the persistent queue worker continues with the next queued rule. Cruster itself cannot recover from process aborts, segmentation faults, out-of-memory termination, or undefined behavior that terminates the whole process; those require process-level supervision outside Cruster.

Passive rules may inspect the pair but cannot send requests. Calling `send()` from a passive rule raises `PermissionError`. Active rules may clone and modify requests and call `send()` multiple times.

### `HttpHeader`

Headers are represented as dictionaries:

```python
class HttpHeader(TypedDict):
    name: str
    value: str
```

- `name` contains the HTTP header name.
- `value` is always Base64-encoded. Decode it before textual or binary inspection and Base64-encode replacement values.
- Headers are a list rather than a mapping so duplicate header names are preserved.
- Header order should not be used as a security signal because the Rust HTTP stack can normalize it.

Example:

```python
import base64

for header in response.headers:
    if header["name"].lower() == "content-type":
        content_type = base64.b64decode(header["value"]).decode(
```

### `HttpRequest`

```python
class HttpRequest:
    method: str
    uri: str
    version: str
    headers: list[HttpHeader]
    body: str

    def clone(self) -> HttpRequest: ...
    def to_dict(self) -> HttpRequestDict: ...
```

Fields:

- `method` is an HTTP method such as `GET` or `POST`. Before sending, Rust validates it as a `reqwest::Method`.
- `uri` is the absolute request URL used by the sender, for example `https://example.org/path?name=value`. Rust validates it as a URL before sending.
- `version` is the source HTTP version string. Supported sender values are `HTTP/0.9`, `HTTP/1.0`, `HTTP/1.1`, `HTTP/2`, `HTTP/2.0`, `HTTP/3`, and `HTTP/3.0`. HTTP/2 and HTTP/3 requests are currently sent as HTTP/1.1 by the underlying Cruster sender.
- `headers` is a mutable list of `HttpHeader` dictionaries. Names and decoded values are validated by Rust when `send()` is called.
- `body` is a Base64-encoded string. This preserves arbitrary binary request bodies. An empty body is  represented by an empty string.

Methods:

- `clone()` returns a new `HttpRequest` with a separate top-level representation suitable for mutation before an active probe. Rules should modify the clone rather than the original request. The current implementation serializes the request through `to_dict()`; replacing scalar fields is isolated, but nested header dictionaries should be copied explicitly before in-place modification if the original must remain unchanged.
- `to_dict()` returns the serialization shape accepted by the native sender. It is primarily used by the runtime and by `exchange()`; normal rules rarely need to call it directly.

Request bodies are decoded and changed as follows:

```python
import base64

changed = request.clone()
body = base64.b64decode(changed.body)
changed.body = base64.b64encode(body + b"&probe=true").decode("ascii")
```

### `HttpResponse`

```python
class HttpResponse:
    status: str
    version: str
    headers: list[HttpHeader]
    body: str

    def to_dict(self) -> HttpResponseDict: ...
```

Fields:

- `status` contains the complete status representation emitted by the Rust HTTP library, for example `200 OK`.
- `version` contains the HTTP version string.
- `headers` follows the same Base64 and duplicate-preserving contract as request headers.
- `body` is Base64-encoded and can contain arbitrary bytes.

`to_dict()` returns the serialization shape used when an exchange is attached to a finding.

### `send()`

```python
def send(request: HttpRequest, max_redirects: int = 5) -> HttpResponse: ...
```

`send()` is the native Rust bridge available to active rules.

Contract:

1. The request object is serialized by the embedded API.
2. Rust validates the method, absolute URL, HTTP version, Base64 body, header names, and Base64-decoded header values.
3. Cruster sends the request with its Rust HTTP sender and returns a fully buffered `HttpResponse`.
4. Redirects are followed manually up to `max_redirects`. The default is `5`. A negative or non-integer value is rejected by Python/Rust argument conversion.
5. Network, URL, HTTP construction, decoding, or redirect errors are raised as `RuntimeError` or `ValueError` inside the rule. An uncaught exception marks that rule execution as failed and includes its Python traceback.

Important behavioral details:

- `send()` performs a real network operation and can change remote state.
- It may be called multiple times by one active rule.
- The current sender uses rustls, disables automatic reqwest redirects, and sends HTTP/1 requests.
- Cookies from the initial probe request are forwarded during redirect following; redirects are sent as `GET` requests.
- The response body is fully buffered before control returns to Python.
- There is currently no per-rule request limit or script execution timeout. Only trusted active rules should be executed.

Example:

```python
changed = request.clone()
changed.uri += ("&" if "?" in changed.uri else "?") + "probe=1"
probe_response = send(changed, max_redirects=2)
```

### `finding()`

```python
def finding(
    name: str,
    extracted: list[str] | None = None,
    exchanges: list[Exchange] | None = None,
) -> Finding: ...
```

Creates one finding dictionary in the exact shape consumed by Rust.

- `name` is the finding identifier within the rule. Findings with the same name are stored in a map, so later duplicates overwrite earlier findings from the same execution. Use unique names when returning multiple findings.
- `extracted` contains human-readable evidence such as matched values, affected parameters, or payload markers. It defaults to an empty list.
- `exchanges` contains active HTTP evidence produced with `exchange()`. It defaults to an empty list and is normally empty for passive rules.

The result of `finding()` must be returned from `scan()` as an element of the result list.

### `exchange()`

```python
def exchange(
    request: HttpRequest,
    response: HttpResponse,
    payloads: list[str] | None = None,
) -> Exchange: ...
```

Creates serializable evidence for an active finding.

- `request` should be the exact modified request passed to `send()`.
- `response` should be the corresponding response returned by `send()`.
- `payloads` identifies payload strings responsible for the probe. It defaults to an empty list.

`exchange()` does not send a request and does not verify that its request and response correspond to each other. It only packages evidence for result output. Invalid Base64 or invalid headers in an exchange cannot be reconstructed by Rust; such malformed evidence should not be returned.

### `log()`

```python
def log(message: object, level: str = "info") -> None: ...
```

Emits an informational or diagnostic message from a Python rule through Cruster's native audit logging channel.

- `message` accepts any Python object. Cruster converts it with `str()` before passing it to Rust.
- `level` is optional and defaults to `info`. Supported values are `debug`, `info`, `warning`, and `error`; any other value raises `ValueError`.
- Calls are collected during rule execution and forwarded to the audit coordinator after the rule returns.
- Rule logs are printed only when audit verbose output is enabled with `--verbose`/`-v`. Calling `log()` without verbose mode is valid, but produces no terminal output.
- Every printed line is marked as a rule log and includes its level, worker number, rule ID, and HTTP pair ID. Example: `[rule log][DEBUG][worker 2][rule 'python-detect-google-host'][pair 7] checking hostname google.com`.
- Logs are diagnostic output only. They are not findings and are not written to the audit findings JSONL file.
- Logging is available to both passive and active rules and has no network side effects.

Example:

```python
from cruster import log

log("starting response analysis", level="debug")
log(f"detected marker {marker}")
```

### `Finding` and `Exchange` result shapes

For reference, helper functions produce these dictionary shapes:

```python
class Exchange(TypedDict):
    request: HttpRequestDict
    response: HttpResponseDict
    payloads: list[str]


class Finding(TypedDict):
    name: str
    extracted: list[str]
    exchanges: list[Exchange]
```

Use `finding()` and `exchange()` instead of constructing these dictionaries manually. The helpers keep rules aligned with the Rust deserialization contract and provide editor type checking.

### Errors and lifecycle

- Metadata is read statically from the leading JSON-compatible object and does not start RustPython. A rule batch uses one RustPython interpreter and cached compiled code, but each scan executes that code in a fresh scope, so module-level mutable state is not persistent between HTTP pairs.
- The rule source is read again for each execution, so edits are observed by subsequent scans.
- Syntax errors, import errors, uncaught rule exceptions, invalid return values, native send errors, and invalid serialized HTTP data produce a failed rule state rather than a finding.
- Python exceptions are rendered with traceback information in Cruster's failure output.
- Missing requests or responses cause the rule execution to be skipped before Python is called.
- The embedded interpreter is not a security sandbox. Rule code has access to the RustPython standard library shipped with Cruster.

## Complete active rule example

```python
metadata = {
    "id": "reflected-probe",
    "about": "Checks whether a query marker is reflected",
    "authors": ["Cruster user"],
    "references": [],
    "tags": ["reflection", "active"],
    "severity": "medium",
    "type": "active"
}

import base64

from cruster import HttpRequest, HttpResponse, exchange, finding, send


def scan(request: HttpRequest, response: HttpResponse):
    marker = "cruster-marker-7391"
    changed = request.clone()
    separator = "&" if "?" in changed.uri else "?"
    changed.uri += separator + "probe=" + marker

    probe = send(changed, max_redirects=2)
    probe_body = base64.b64decode(probe.body).decode("utf-8", errors="replace")
    if marker not in probe_body:
        return []

    return [
        finding(
            "reflected-probe",
            extracted=[marker],
            exchanges=[exchange(changed, probe, payloads=[marker])],
        )
    ]
```

## Included example rules

- `rules/python/passive/security_headers.py` demonstrates passive response-header inspection.
- `rules/python/passive/detect_google_host.py` demonstrates passive hostname detection and matches `google.com` plus its subdomains without matching look-alike domains such as `notgoogle.com`.
- `rules/python/active/reflected_marker.py` demonstrates request cloning, native sending, response comparison, and exchange evidence.
