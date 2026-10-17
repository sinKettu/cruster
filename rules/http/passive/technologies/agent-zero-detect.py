metadata = {
    "id": "agent-zero-detect",
    "about": "Agent-Zero Application - Detect (passive Nuclei technology fingerprint)",
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

    if "Agent Zero" not in body or "a0version" not in body:
        return []

    versions = [
        match.group(1)
        for match in re.finditer(r"v([0-9]+\.[0-9]+\.[0-9]+(?:\.[0-9]+)?)", body)
    ]
    return [finding("agent-zero-detect", extracted=versions)]
