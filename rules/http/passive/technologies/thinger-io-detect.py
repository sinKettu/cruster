metadata = {
    "id": "thinger-io-detect",
    "about": "Thinger.io - Detect (passive Nuclei technology fingerprint)",
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
    status_code = response.status.lstrip().split(" ", 1)[0]
    if status_code != "200":
        return []

    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []

    titles = (
        "<title>Thinger.io | Cloud Console</title>",
        "<title>Thinger.io - Cloud Console</title>",
    )
    if not any(title in body for title in titles):
        return []
    return [finding("thinger-io-detect")]
