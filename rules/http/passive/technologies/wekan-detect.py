metadata = {
    "id": "wekan-detect",
    "about": "Wekan - Detect (passive Nuclei technology fingerprint)",
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
    if request.uri.split("?", 1)[0].rstrip("/").endswith(":"):
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []
    markers = ["<title>Wekan</title>", "/packages/wekan-"]
    if not response.status.startswith("200") or not all(
        marker in body for marker in markers
    ):
        return []
    return [finding("wekan-detect")]
