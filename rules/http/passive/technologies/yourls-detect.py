metadata = {
    "id": "yourls-detect",
    "about": "YOURLS - Detection (passive Nuclei technology fingerprint)",
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
    if not status or int(status.group(1)) != 200:
        return []

    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []

    markers = ("<title>YOURLS", 'content="YOURLS', 'alt="YOURLS', "/yourls-logo")
    if not any(marker in body for marker in markers):
        return []

    versions = []
    patterns = (r"YOURLS<\/a>([ a-z0-9.-]+)", r"content=\"YOURLS([ 0-9.]+)\"")
    for pattern in patterns:
        versions.extend(match.group(1) for match in re.finditer(pattern, body))

    return [finding("yourls-detect", extracted=versions)]
