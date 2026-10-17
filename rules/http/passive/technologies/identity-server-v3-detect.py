metadata = {
    "id": "identity-server-v3-detect",
    "about": "Identity Server V3 - Detect (passive Nuclei technology fingerprint)",
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
    status = re.match(r"\s*(\d+)", response.status)
    if not status or int(status.group(1)) != 200:
        return []

    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""

    if "IdentityServer v3" not in body or "identityserver.github.io" not in body:
        return []

    extracted = [
        match.group(1) for match in re.finditer(r"(?i)build\s+([a-z0-9.-]+)", body)
    ]
    return [finding("identity-server-v3-detect", extracted=extracted)]
