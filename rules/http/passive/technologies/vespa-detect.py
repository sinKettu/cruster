metadata = {
    "id": "vespa-detect",
    "about": "Vespa - Detect (passive Nuclei technology fingerprint)",
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
    if not request.uri.split("?", 1)[0].endswith("/ApplicationStatus"):
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []
    if '"vespa"' not in body or '"version"' not in body:
        return []

    pattern = r'"vespa"\s*:\s*\{\s*"version"\s*:\s*"([0-9\.]+)"'
    extracted = [match.group(1) for match in re.finditer(pattern, body)]
    return [finding("vespa-detect", extracted=extracted)]
