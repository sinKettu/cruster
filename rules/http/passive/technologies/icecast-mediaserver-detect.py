metadata = {
    "id": "icecast-mediaserver-detect",
    "about": "Icecast Streaming Media Server Information Panel - Detect (passive Nuclei technology fingerprint)",
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
    if target != "/server_version.xsl":
        return []

    status = re.match(r"\s*(\d+)", response.status)
    if not status or int(status.group(1)) != 200:
        return []

    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""

    if "Icecast Streaming Media Server" not in body:
        return []

    extracted = [match.group(0) for match in re.finditer(r"Icecast ([0-9.]+)", body)]
    return [
        finding("icecast-mediaserver-detect", extracted=list(dict.fromkeys(extracted)))
    ]
