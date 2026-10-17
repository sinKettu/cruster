metadata = {
    "id": "typesense-dashboard-detect",
    "about": "Typesense Dashboard - Detect (passive Nuclei technology fingerprint)",
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


def scan(_request: HttpRequest, response: HttpResponse):
    if response.status.lstrip().split(" ", 1)[0] != "200":
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []
    words = (
        "<title>Typesense-Dashboard</title>",
        'content="A Typesense Dashboard to manage and browse collections."',
    )
    if not all(word in body for word in words):
        return []
    return [finding("typesense-dashboard-detect")]
