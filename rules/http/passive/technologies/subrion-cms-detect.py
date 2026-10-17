metadata = {
    "id": "subrion-cms-detect",
    "about": "Subrion CMS Detect (passive Nuclei technology fingerprint)",
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
    marker_match = (
        'subrion.org" title="Open Source CMS">Subrion CMS' in body
        or "Subrion CMS - Open Source Content Management System" in body
    )
    matched = status is not None and int(status.group(1)) == 200 and marker_match
    return [finding("subrion-cms-detect", extracted=[])] if matched else []
