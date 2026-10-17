metadata = {
    "id": "aptabase-detect",
    "about": "Aptabase - Detect (passive Nuclei technology fingerprint)",
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
    status = re.match(r"\s*(\d+)", response.status)
    if status is None or int(status.group(1)) != 200:
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []

    markers = (
        "<title>Aptabase</title>",
        'content="Open Source, Privacy-First and Simple Analytics for Mobile, Desktop and Web apps."',
        '<meta name="twitter:site" content="@aptabase"',
    )
    if not all(marker in body for marker in markers):
        return []

    return [finding("aptabase-detect")]
