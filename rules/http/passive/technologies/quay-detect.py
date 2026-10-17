metadata = {
    "id": "quay-detect",
    "about": "Quay - Detect (passive Nuclei technology fingerprint)",
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

    status_match = re.match(r"\s*(\d+)", response.status)
    if status_match is None:
        return []

    if int(status_match.group(1)) != 200:
        return []

    markers = (
        '<html ng-app="quay"',
        "/static/img/quay_favicon.png",
        "window.__endpoints",
    )
    if not all(marker in body for marker in markers):
        return []

    return [finding("quay-detect", extracted=[])]
