metadata = {
    "id": "zurb-foundation-detect",
    "about": "ZURB Foundation Framework - Detect (passive Nuclei technology fingerprint)",
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
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []

    asset_pattern = r"(?i)foundation(?:\.min)?\.(?:css|js)"
    if not re.search(asset_pattern, body):
        return []

    return [finding("zurb-foundation-detect")]
