metadata = {
    "id": "langflow-detect",
    "about": "Langflow - Detect (passive Nuclei technology fingerprint)",
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
    if '"package":"Langflow"' not in body and "<title>Langflow</title>" not in body:
        return []
    extracted = [
        match.group(1) for match in re.finditer(r'"version"\s*:\s*"([0-9\.]+)"', body)
    ]
    return [finding("langflow-detect", extracted=extracted)]
