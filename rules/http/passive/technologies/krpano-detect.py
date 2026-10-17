metadata = {
    "id": "krpano-detect",
    "about": "Krpano Panorama Viewer - Detection (passive Nuclei technology fingerprint)",
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
    del request
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""

    required_markers = ("embedpano({", "passQueryParameters:true", "swf")
    if not all(marker in body for marker in required_markers):
        return []

    return [finding("krpano-detect")]
