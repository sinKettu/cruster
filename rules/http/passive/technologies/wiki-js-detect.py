metadata = {
    "id": "wiki-js-detect",
    "about": "Wiki.js - Detect (passive Nuclei technology fingerprint)",
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
    if not request.uri.split("?", 1)[0].endswith("/_assets/js/app.js"):
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1").lower()
    except Exception:
        return []
    if not response.status.startswith("200"):
        return []
    if "wiki.js - wiki.js.org" not in body or "window.wiki" not in body:
        return []
    return [finding("wiki-js-detect")]
