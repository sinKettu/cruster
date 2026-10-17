metadata = {
    "id": "apple-httpserver",
    "about": "Apple HttpServer Detect (passive Nuclei technology fingerprint)",
    "authors": ["ProjectDiscovery contributors", "Cruster"],
    "references": [
        "https://github.com/projectdiscovery/nuclei-templates/tree/main/http/technologies"
    ],
    "tags": ["technology", "discovery", "nuclei", "passive"],
    "severity": "info",
    "type": "passive",
}

import base64
from cruster import HttpRequest, HttpResponse, finding


def scan(request: HttpRequest, response: HttpResponse):
    try:
        headers = "\n".join(
            header["name"] + ": " + base64.b64decode(header["value"]).decode("latin-1")
            for header in response.headers
        )
    except Exception:
        return []

    markers = (
        "X-Apple-Jingle-Correlation-Key",
        "AppleHttpServer",
        "X-Apple-Request-UUID",
    )
    if not any(marker in headers for marker in markers):
        return []

    return [finding("apple-httpserver")]
