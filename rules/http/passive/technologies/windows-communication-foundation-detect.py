metadata = {
    "id": "windows-communication-foundation-detect",
    "about": "Windows Communication Foundation - Detect (passive Nuclei technology fingerprint)",
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
        body = base64.b64decode(response.body).decode("latin-1").lower()
    except Exception:
        return []
    markers = [
        "endpoint not found.",
        'class="heading1">service',
        "this is a windows&#169; communication foundation service.",
    ]
    if not response.status.startswith("200"):
        return []
    if "<title>service</title>" not in body:
        return []
    if not any(marker in body for marker in markers):
        return []
    return [finding("windows-communication-foundation-detect")]
