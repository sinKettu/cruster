# Python audit rules

Cruster executes Python rules inside the RustPython interpreter embedded in the binary. Python is not installed or spawned on the host.

Every rule is a `.py` file with a `metadata` dictionary and a `scan(request, response)` entry point. `metadata` contains `id`, `about`, `authors`, `references`, `tags`, `severity` (`info`, `low`, `medium`, or `high`) and `type` (`passive` or `active`).

Requests expose `method`, `uri`, `version`, `headers`, and `body`; responses expose `status`, `version`, `headers`, and `body`. Headers are lists of `{name, value}` dictionaries and values are Base64 strings, preserving duplicate and binary headers. Bodies are Base64 strings. A request has `clone()` and both message types have `to_dict()`.

Return a list created with `finding(name, extracted=[], exchanges=[])`. Returning `[]` means the rule found nothing.

Active rules additionally call `send(request, max_redirects=5)`. This native bridge validates the modified request in Rust, sends it with Cruster's HTTP sender, and returns an `HttpResponse`. Evidence can be attached with `exchange(request, response, payloads=[])`.

Rules are discovered recursively from configured rule paths. Include/exclude IDs, tags, paths, and active/passive switches apply to Python rules. YAML rules and their implementation remain part of the project, but their execution is temporarily disabled while the YAML rule design is being reconsidered: the loader prints a warning and skips them, while direct execution is guarded by an explicit `todo!()`.

The embedded interpreter is an extension boundary, not a security sandbox. Only load trusted rules. Active rules perform real network requests and must be enabled explicitly.
