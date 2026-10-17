metadata = {
    "id": "solaranzeige-detect",
    "about": "Solaranzeige - Detect (passive Nuclei technology fingerprint)",
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
    status = re.match(r"\s*(\d+)", response.status)
    title_match = (
        "<title>Open Source Projekt Solaranzeige</title>" in body
        or "<title>Solaranzeige</title>" in body
    )
    matched = status is not None and int(status.group(1)) == 200 and title_match
    return [finding("solaranzeige-detect", extracted=[])] if matched else []
