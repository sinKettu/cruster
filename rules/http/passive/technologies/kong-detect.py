metadata = {
    "id": "kong-detect",
    "about": "Detect Kong (passive Nuclei technology fingerprint)",
    "authors": ["ProjectDiscovery contributors", "Cruster"],
    "references": [
        "https://github.com/projectdiscovery/nuclei-templates/tree/main/http/technologies"
    ],
    "tags": ["technology", "discovery", "nuclei", "passive"],
    "severity": "info",
    "type": "passive"
}

import base64

from cruster import HttpRequest, HttpResponse, finding


def scan(request: HttpRequest, response: HttpResponse):
    del request
    header_lines = []
    server_values = []
    for header in response.headers:
        try:
            value = base64.b64decode(header["value"]).decode("latin-1")
        except Exception:
            continue
        header_lines.append(header["name"] + ": " + value)
        if header["name"].lower() == "server":
            server_values.append(value)

    headers = "\n".join(header_lines).lower()
    markers = (
        "server: kong",
        "x-kong-response-latency",
        "x-kong-upstream-latency",
        "x-kong-proxy-latency",
        "x-kong-admin-request-id",
    )
    if not any(marker in headers for marker in markers):
        return []

    extracted = []
    if server_values:
        extracted.append("\n".join(server_values))

    return [finding("kong-detect", extracted=extracted)]
