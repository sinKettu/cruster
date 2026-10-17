metadata = {
    "id": "splash-rendering-service",
    "about": "Splash - Detect (passive Nuclei technology fingerprint)",
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
    status = re.match(r"\s*(\d+)", response.status)
    matched = (
        status is not None
        and int(status.group(1)) == 200
        and "<title>Splash" in body
        and "splash.loadExample" in body
    )
    if not matched:
        return []
    versions = [match.group(1) for match in re.finditer(r"Splash v([0-9.]+)", body)]
    return [finding("splash-rendering-service", extracted=versions)]
