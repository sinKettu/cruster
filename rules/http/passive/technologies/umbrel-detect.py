metadata = {
    "id": "umbrel-detect",
    "about": "Umbrel - Detect (passive Nuclei technology fingerprint)",
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


def scan(_request: HttpRequest, response: HttpResponse):
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []

    secondary_markers = [
        "<h1>umbrelOS</h1>",
        "Umbrel doesn't work properly without JavaScript enabled",
    ]
    if not response.status.startswith("200"):
        return []
    if "<title>Umbrel</title>" not in body:
        return []
    if not any(marker in body for marker in secondary_markers):
        return []
    return [finding("umbrel-detect")]
