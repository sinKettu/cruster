metadata = {
    "id": "akkoma-detect",
    "about": "Akkoma - Detect (passive Nuclei technology fingerprint)",
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
    target = target.split("#", 1)[0].split("?", 1)[0]
    if target != "/api/v1/instance":
        return []

    status = re.match(r"\s*(\d+)", response.status)
    if status is None or int(status.group(1)) != 200:
        return []

    content_type = ""
    for header in response.headers:
        if header["name"].lower() == "content-type":
            try:
                content_type = base64.b64decode(header["value"]).decode("latin-1")
            except Exception:
                return []
            break
    if "application/json" not in content_type:
        return []

    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []
    if '"version"' not in body or "compatible; Akkoma " not in body:
        return []

    return [finding("akkoma-detect")]
