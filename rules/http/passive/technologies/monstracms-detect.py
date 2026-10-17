metadata = {
    "id": "monstracms-detect",
    "about": "MonstraCMS Detection (passive Nuclei technology fingerprint)",
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
    body = base64.b64decode(response.body).decode("latin-1")
    status = re.match(r"\s*(\d+)", response.status)
    if (
        not status
        or int(status.group(1)) != 200
        or 'content="Powered by Monstra' not in body
    ):
        return []
    versions = [
        match.group(1)
        for match in re.finditer(r'content="Powered by Monstra ([0-9.]+)', body)
    ]
    return [finding("monstracms-detect", extracted=versions)]
