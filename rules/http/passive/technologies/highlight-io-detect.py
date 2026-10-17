metadata = {
    "id": "highlight-io-detect",
    "about": "Highlight.io - Detect (passive Nuclei technology fingerprint)",
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
    status_match = re.match(r"\s*(\d+)", response.status)
    if status_match is None or int(status_match.group(1)) != 200:
        return []

    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []

    markers = (
        "<title>highlight.io</title>",
        "highlight.io | The Open Source, Fullstack Monitoring Platform",
        "@highlight-run/rrweb",
    )
    if not all(marker in body for marker in markers):
        return []

    return [finding("highlight-io-detect")]
