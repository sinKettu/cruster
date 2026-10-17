metadata = {
    "id": "gollum-detect",
    "about": "Gollum - Detect (passive Nuclei technology fingerprint)",
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
    status = re.match(r"\s*(\d+)", response.status)
    if status is None or int(status.group(1)) != 200:
        return []

    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []

    if "/gollum/assets/" not in body:
        return []

    path_markers = (
        "var pageFullPath =",
        "var pageFullPath   =",
    )
    if not any(marker in body for marker in path_markers):
        return []

    return [finding("gollum-detect")]
