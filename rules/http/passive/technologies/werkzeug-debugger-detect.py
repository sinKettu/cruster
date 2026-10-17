metadata = {
    "id": "werkzeug-debugger-detect",
    "about": "Werkzeug debugger console (passive Nuclei technology fingerprint)",
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
    if not request.uri.split("?", 1)[0].endswith("/console"):
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []
    if "<h1>Interactive Console</h1>" not in body:
        return []
    return [finding("werkzeug-debugger-detect")]
