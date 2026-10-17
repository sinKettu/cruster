metadata = {
    "id": "zope-detect",
    "about": "Zope Quick Start Detect (passive Nuclei technology fingerprint)",
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
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""

    if "<title>Zope QuickStart</title>" in body:
        return [finding("zope-detect")]

    for item in response.headers:
        if item["name"].lower() != "server":
            continue
        try:
            value = base64.b64decode(item["value"]).decode("latin-1")
        except Exception:
            continue
        if "Zope/" in value:
            return [finding("zope-detect")]

    return []
