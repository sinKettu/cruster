metadata = {
    "id": "a-blog-cms-detect",
    "about": "a-blog cms - Detect (passive Nuclei technology fingerprint)",
    "authors": ["ProjectDiscovery contributors", "Cruster"],
    "references": [
        "https://github.com/projectdiscovery/nuclei-templates/tree/main/http/technologies"
    ],
    "tags": ["technology", "discovery", "nuclei", "passive"],
    "severity": "info",
    "type": "passive"
}

import base64
import re

from cruster import HttpRequest, HttpResponse, finding


def scan(request: HttpRequest, response: HttpResponse):
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []
    if "a-blog cms" not in body or "acms-js" not in body:
        return []
    versions = [
        match.group(1)
        for match in re.finditer(r'(?i)content="a-blog cms (?:Ver\.|v)?([0-9.]+)', body)
    ]
    return [finding("a-blog-cms-detect", extracted=versions)]
