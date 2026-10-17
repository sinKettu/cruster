metadata = {
    "id": "parseable-detect",
    "about": "Parseable - Detect (passive Nuclei technology fingerprint)",
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
    title = re.search(
        r"(?i)<title>Parseable(?: Log Storage| \| (?:Fast )?Observability[^<]*)</title>",
        body,
    )
    signatures = (
        "Fast Observability on Object Store",
        "Observability for agents, apps and systems",
        "Parseable Log Storage",
    )
    text = any(value in body for value in signatures)
    detected = (
        status is not None
        and int(status.group(1)) == 200
        and title is not None
        and text
    )
    return [finding("parseable-detect", extracted=[])] if detected else []
