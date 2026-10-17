metadata = {
    "id": "keystonejs-detect",
    "about": "KeystoneJS - Detect (passive Nuclei technology fingerprint)",
    "authors": ["ProjectDiscovery contributors", "Cruster"],
    "references": [
        "https://github.com/projectdiscovery/nuclei-templates/tree/main/http/technologies"
    ],
    "tags": ["technology", "discovery", "nuclei", "passive"],
    "severity": "info",
    "type": "passive",
}

import base64
import re

from cruster import HttpRequest, HttpResponse, finding


def scan(request: HttpRequest, response: HttpResponse):
    del request
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""

    status_match = re.match(r"\s*(\d+)", response.status)
    status_code = int(status_match.group(1)) if status_match else 0
    has_title = "<title>KeystoneJS</title>" in body
    has_product_marker = "Welcome to KeystoneJS" in body or "/admin/js/main." in body
    if not (has_title and has_product_marker and status_code == 200):
        return []

    return [finding("keystonejs-detect")]
