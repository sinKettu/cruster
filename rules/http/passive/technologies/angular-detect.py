metadata = {
    "id": "angular-detect",
    "about": "Angular detect (passive Nuclei technology fingerprint)",
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
    if "ng-version=" not in body:
        return []
    versions = list(dict.fromkeys(re.findall(r'ng-version="([0-9.]+)"', body)))
    return [finding("angular-detect", extracted=versions)]
