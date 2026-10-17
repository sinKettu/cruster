metadata = {
    "id": "apache-inlong-detect",
    "about": "Apache InLong - Detect (passive Nuclei technology fingerprint)",
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
    if not response.status.startswith("200"):
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1").lower()
    except Exception:
        body = ""

    if "inlong</title>" not in body and "webpackjsonpinlong" not in body:
        return []

    return [finding("apache-inlong-detect")]
