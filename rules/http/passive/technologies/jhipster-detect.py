metadata = {
    "id": "jhipster-detect",
    "about": "JHipster Detect (passive Nuclei technology fingerprint)",
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
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""

    markers = ("JHipster", "@jhipster", "Welcome, Java Hipster!", "jhipster-error")
    if not any(marker in body for marker in markers):
        return []

    return [finding("jhipster-detect")]
