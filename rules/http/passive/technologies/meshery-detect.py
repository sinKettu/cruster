metadata = {
    "id": "meshery-detect",
    "about": "Meshery - Detect (passive Nuclei technology fingerprint)",
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
    target = target[target.find("/") :] if "/" in target else "/"
    target = target.split("#", 1)[0].split("?", 1)[0] or "/"
    if target != "/api/providers":
        return []

    body = base64.b64decode(response.body).decode("latin-1")
    status = re.match(r"\s*(\d+)", response.status)
    lowered = body.lower()
    if not (
        status
        and int(status.group(1)) == 200
        and "persist-meshery-" in lowered
        and "ismesheryuirestricted" in lowered
    ):
        return []
    versions = [
        match.group(1)
        for match in re.finditer(r'(?i)"package_version":\s*"([a-z0-9.]+)"', body)
    ]
    return [finding("meshery-detect", extracted=versions)]
