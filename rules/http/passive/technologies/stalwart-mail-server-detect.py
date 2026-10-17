metadata = {
    "id": "stalwart-mail-server-detect",
    "about": "Stalwart Mail Server - Detect (passive Nuclei technology fingerprint)",
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
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    status = re.match(r"\s*(\d+)", response.status)
    matched = (
        status is not None
        and int(status.group(1)) == 200
        and "<title>Stalwart Management</title>" in body
        and "TrunkApplicationStarted" in body
        and "_bg.wasm" in body
    )
    return [finding("stalwart-mail-server-detect", extracted=[])] if matched else []
