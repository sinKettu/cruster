metadata = {
    "id": "vaadin-detect",
    "about": "Vaadin Framework - Detect (passive Nuclei technology fingerprint)",
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


def scan(_request: HttpRequest, response: HttpResponse):
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []

    markers = [
        "/vaadin/",
        "v-generated-body",
        "v-app-loading",
        "vaadinbootstrap",
        "vaadin.",
        "vaadindirect",
    ]
    if not any(marker in body.lower() for marker in markers):
        return []

    patterns = [r"vaadinBootstrap\.js\?v=([0-9.]+)", r'"vaadinVersion":"([0-9.]+)"']
    extracted = []
    for pattern in patterns:
        extracted.extend(match.group(1) for match in re.finditer(pattern, body))
    return [finding("vaadin-detect", extracted=extracted)]
