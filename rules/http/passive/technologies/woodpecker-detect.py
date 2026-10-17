metadata = {
    "id": "woodpecker-detect",
    "about": "Woodpecker - Detect (passive Nuclei technology fingerprint)",
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
    if not request.uri.split("?", 1)[0].endswith("/web-config.js"):
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []
    if (
        not response.status.startswith("200")
        or "window.woodpecker_" not in body.lower()
    ):
        return []
    pattern = r'(?i)window.WOODPECKER_VERSION\s*=\s*"([0-9.]+)"'
    extracted = [match.group(1) for match in re.finditer(pattern, body)]
    return [finding("woodpecker-detect", extracted=extracted)]
