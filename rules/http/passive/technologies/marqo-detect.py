metadata = {
    "id": "marqo-detect",
    "about": "Marqo Vector Search Engine - Detect (passive Nuclei technology fingerprint)",
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
    body = base64.b64decode(response.body).decode("latin-1")
    status = re.match(r"\s*(\d+)", response.status)
    if not status or int(status.group(1)) != 200 or "Welcome to Marqo" not in body:
        return []
    versions = [match.group(1) for match in re.finditer(r'"version":"([^"]+)"', body)]
    return [finding("marqo-detect", extracted=versions)]
