metadata = {
    "id": "winter-cms-detect",
    "about": "Winter CMS - Detect (passive Nuclei technology fingerprint)",
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
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []
    markers = ['<meta name="generator" content="Winter CMS"', "/themes/"]
    if not response.status.startswith("200") or not all(
        marker in body for marker in markers
    ):
        return []
    return [finding("winter-cms-detect")]
