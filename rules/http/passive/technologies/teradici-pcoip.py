metadata = {
    "id": "teradici-pcoip",
    "about": "Teradici PCoIP Detection (passive Nuclei technology fingerprint)",
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
    target = request.uri.split("://", 1)[-1]
    if "/" in target:
        target = target[target.find("/") :]
    else:
        target = "/"
    target = target.split("#", 1)[0].split("?", 1)[0]
    if target != "/info/":
        return []

    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []

    if "PCoIP Connection Manager" not in body:
        return []

    extracted = []
    for match in re.finditer(r"PCoIP Connection Manager/([0-9.]+)\.", body):
        extracted.append(match.group(1))
    return [finding("teradici-pcoip", extracted=extracted)]
