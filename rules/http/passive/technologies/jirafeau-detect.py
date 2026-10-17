metadata = {
    "id": "jirafeau-detect",
    "about": "Jirafeau - Detect (passive Nuclei technology fingerprint)",
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

    status_match = re.match(r"\s*(\d+)", response.status)
    if not status_match or int(status_match.group(1)) != 200:
        return []
    if "media/courgette/favicon.ico" not in body:
        return []
    if not re.search(r"<title>Jirafeau[^<]*</title>", body, re.IGNORECASE):
        return []

    return [finding("jirafeau-detect")]
