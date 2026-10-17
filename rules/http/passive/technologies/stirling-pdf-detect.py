metadata = {
    "id": "stirling-pdf-detect",
    "about": "Stirling PDF - Detect (passive Nuclei technology fingerprint)",
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
    status = re.match(r"\s*(\d+)", response.status)
    marker_match = any(
        value in body
        for value in (
            "<title>Stirling PDF",
            "<title>Stirling-PDF",
            'content="Stirling PDF',
            "window.stirlingPDF",
        )
    )
    matched = status is not None and int(status.group(1)) == 200 and marker_match
    return [finding("stirling-pdf-detect", extracted=[])] if matched else []
