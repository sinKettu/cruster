metadata = {
    "id": "semantic-ui-detect",
    "about": "Semantic UI Framework - Detect (passive Nuclei technology fingerprint)",
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
        body = ""
    patterns = (
        r"(?i)href=[\"'][^\"']*semantic(?:[\.-]min)?\.css",
        r"(?i)src=[\"'][^\"']*semantic(?:[\.-]min)?\.js",
    )
    if not any(re.search(pattern, body) is not None for pattern in patterns):
        return []
    return [finding("semantic-ui-detect", extracted=[])]
