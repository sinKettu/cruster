metadata = {
    "id": "blazor-webassembly-detect",
    "about": "Blazor WebAssembly - Detect (passive Nuclei technology fingerprint)",
    "authors": ["ProjectDiscovery contributors", "Cruster"],
    "references": [
        "https://github.com/projectdiscovery/nuclei-templates/tree/main/http/technologies"
    ],
    "tags": ["technology", "discovery", "nuclei", "passive"],
    "severity": "info",
    "type": "passive",
}
import base64
import re

from cruster import HttpRequest, HttpResponse, finding


def scan(request: HttpRequest, response: HttpResponse):
    target = request.uri.split("://", 1)[-1]
    target = target[target.find("/") :] if "/" in target else "/"
    target = target.split("#", 1)[0].split("?", 1)[0] or "/"
    if target != "/_framework/blazor.boot.json":
        return []

    status = re.match(r"\s*(\d+)", response.status)
    if not status or int(status.group(1)) != 200:
        return []

    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []

    lower_body = body.lower()
    if not all(
        marker in lower_body
        for marker in (
            "dotnet.wasm",
            '"entryassembly"',
            '"microsoft.jsinterop.webassembly.dll"',
        )
    ):
        return []

    extracted = [
        match.group(1) for match in re.finditer(r"dotnet.([0-9.]+).[a-z0-9]+.js", body)
    ]
    return [
        finding(
            "blazor-webassembly-detect",
            extracted=list(dict.fromkeys(extracted)),
        )
    ]
