metadata = {
    "id": "vikunja-detect",
    "about": "Vikunja - Detect (passive Nuclei technology fingerprint)",
    "authors": ["ProjectDiscovery contributors", "Cruster"],
    "references": [
        "https://github.com/projectdiscovery/nuclei-templates/tree/main/http/technologies"
    ],
    "tags": ["technology", "discovery", "nuclei", "passive"],
    "severity": "info",
    "type": "passive"
}

import base64
import json

from cruster import HttpRequest, HttpResponse, finding


def scan(request: HttpRequest, response: HttpResponse):
    if not request.uri.split("?", 1)[0].endswith("/api/v1/info"):
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []

    content_types = []
    for item in response.headers:
        if item["name"].lower() == "content-type":
            try:
                content_types.append(base64.b64decode(item["value"]).decode("latin-1"))
            except Exception:
                continue
    markers = ['"link_sharing_enabled"', '"available_migrators"', '"caldav_enabled"']
    if not response.status.startswith("200"):
        return []
    if not any("application/json" in value for value in content_types):
        return []
    if not all(marker in body for marker in markers):
        return []
    try:
        extracted = [json.dumps(json.loads(body), separators=(",", ":"))]
    except Exception:
        extracted = []
    return [finding("vikunja-detect", extracted=extracted)]
