metadata = {
    "id": "materialize-css-detect",
    "about": "Materialize CSS - Detect (passive Nuclei technology fingerprint)",
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
    body = base64.b64decode(response.body).decode("latin-1")
    patterns = (
        r'(?i)href=["\'][^"\']*materialize(?:[\.-]min)?\.css',
        r'(?i)src=["\'][^"\']*materialize(?:[\.-]min)?\.js',
        r'(?i)class=["\'][^"\']*waves-effect',
        r'(?i)class=["\'][^"\']*materialize-textarea',
    )
    if any(re.search(pattern, body) for pattern in patterns):
        return [finding("materialize-css-detect")]
    return []
