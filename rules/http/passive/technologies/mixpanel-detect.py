metadata = {
    "id": "mixpanel-detect",
    "about": "Mixpanel Analytics - Detect (passive Nuclei technology fingerprint)",
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
    del request
    body = base64.b64decode(response.body).decode("latin-1")
    status = re.match(r"\s*(\d+)", response.status)
    indicators = ("cdn.mxpnl.com", "window.mixpanel", "mixpanel-2-latest.min.js")
    if (
        status
        and int(status.group(1)) == 200
        and all(value in body for value in indicators)
    ):
        return [finding("mixpanel-detect")]
    return []
