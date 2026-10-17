metadata = {
    "id": "mailu-detect",
    "about": "Mailu - Detect (passive Nuclei technology fingerprint)",
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
    if target != "/":
        return []

    body = base64.b64decode(response.body).decode("latin-1")
    status = re.match(r"\s*(\d+)", response.status)
    matched = (
        re.search(r"(?i)<title>Mailu-Admin\s*\|[^<]+</title>", body)
        and 'data-static="/static/"' in body
        and 'href="/static/vendor.css"' in body
    )
    if status and int(status.group(1)) == 200 and matched:
        return [finding("mailu-detect")]
    return []
