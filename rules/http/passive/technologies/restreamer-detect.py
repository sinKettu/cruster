metadata = {
    "id": "restreamer-detect",
    "about": "Restreamer - Detect (passive Nuclei technology fingerprint)",
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
    if status_match is None or int(status_match.group(1)) != 200:
        return []

    required_markers = (
        "<title>Restreamer</title>",
        "datarhei Restreamer logo",
        "Did you mean to go to the admin panel?",
    )
    if not all(marker in body for marker in required_markers):
        return []

    return [finding("restreamer-detect", extracted=[])]
