metadata = {
    "id": "uni-gui-framework",
    "about": "UniGUI Framework - Detect (passive Nuclei technology fingerprint)",
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


def scan(_request: HttpRequest, response: HttpResponse):
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []

    markers = ["uni-xtheme-", "ext-all.js", "uniVars._extVer="]
    version_patterns = [
        r"uni-(\d+\.\d+\.\d+\.\d+)/",
        r'uniVars\._extVer="(\d+\.\d+\.\d+)"',
    ]
    if not any(marker in body for marker in markers):
        return []
    if not any(re.search(pattern, body) for pattern in version_patterns):
        return []

    extracted = []
    for pattern in version_patterns:
        extracted.extend(match.group(1) for match in re.finditer(pattern, body))
    return [finding("uni-gui-framework", extracted=extracted)]
