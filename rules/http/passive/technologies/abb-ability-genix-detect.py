metadata = {
    "id": "abb-ability-genix-detect",
    "about": "ABB Ability Genix Industrial Analytics and AI Suite - Detect (passive Nuclei technology fingerprint)",
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

    markers = (
        "<title>ABB Ability&trade; Genix Industrial Analytics and AI Suite</title>",
        "assets/abb_favicon.ico",
    )
    if not all(marker in body for marker in markers):
        return []

    return [finding("abb-ability-genix-detect")]
