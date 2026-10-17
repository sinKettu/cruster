metadata = {
    "id": "statping-ng-detect",
    "about": "Statping-ng - Detect (passive Nuclei technology fingerprint)",
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
    target = request.uri.split("://", 1)[-1]
    target = target[target.find("/") :] if "/" in target else "/"
    target = target.split("#", 1)[0].split("?", 1)[0] or "/"
    if target != "/":
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    status = re.match(r"\s*(\d+)", response.status)
    title_match = re.search(
        r"<title>[^<]*Statping[^<]* - Service Monitoring</title>", body, re.IGNORECASE
    )
    metadata_match = all(
        value in body
        for value in (
            '<meta name="title" content="',
            "Statping",
            ' - Service Monitoring">',
        )
    )
    matched = (
        status is not None
        and int(status.group(1)) == 200
        and title_match
        and metadata_match
    )
    return [finding("statping-ng-detect", extracted=[])] if matched else []
