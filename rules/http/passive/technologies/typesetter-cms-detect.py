metadata = {
    "id": "typesetter-cms-detect",
    "about": "Typesetter CMS - Detect (passive Nuclei technology fingerprint)",
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


def scan(_request: HttpRequest, response: HttpResponse):
    if response.status.lstrip().split(" ", 1)[0] != "200":
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []
    if '<meta name="generator" content="Typesetter CMS"' not in body:
        return []
    if "/data/_cache/" not in body and "/include/thirdparty/" not in body:
        return []
    return [finding("typesetter-cms-detect")]
