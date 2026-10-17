metadata = {
    "id": "admiralcloud-detect",
    "about": "AdmiralCloud - Detect (passive Nuclei technology fingerprint)",
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
    status = re.match(r"\s*(\d+)", response.status)
    if status is None or int(status.group(1)) != 200:
        return []

    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []

    lowercase_body = body.lower()
    markers = ('content="admiralcloud', "<title>admiralcloud")
    if not any(marker in lowercase_body for marker in markers):
        return []

    versions = [match.group(1) for match in re.finditer(r"\s+([0-9\.]+)</title>", body)]
    return [finding("admiralcloud-detect", extracted=versions)]
