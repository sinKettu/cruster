metadata = {
    "id": "rseenet-detect",
    "about": "Advantech R-SeeNet Detection (passive Nuclei technology fingerprint)",
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
        body = ""

    matches = re.findall(r"<title>R-SeeNet (.*)</title>", body)
    if not matches:
        return []

    return [finding("rseenet-detect", extracted=matches)]
