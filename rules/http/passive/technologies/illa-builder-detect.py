metadata = {
    "id": "illa-builder-detect",
    "about": "ILLA Builder - Detect (passive Nuclei technology fingerprint)",
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
    del request
    status = re.match(r"\s*(\d+)", response.status)
    if not status or int(status.group(1)) != 200:
        return []

    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""

    titles = (
        "<title>ILLA Cloud</title>",
        "ILLA Builder - An open-source low-code Platform for Developers.",
    )
    if not any(title in body for title in titles) or "@illa-design-" not in body:
        return []
    return [finding("illa-builder-detect")]
