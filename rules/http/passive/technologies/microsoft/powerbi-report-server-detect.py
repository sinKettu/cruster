metadata = {
    "id": "powerbi-report-server-detect",
    "about": "PowerBI Report Server - Detect (passive Nuclei technology fingerprint)",
    "authors": [
        "ProjectDiscovery contributors",
        "Cruster"
    ],
    "references": [
        "https://github.com/projectdiscovery/nuclei-templates/tree/main/http/technologies"
    ],
    "tags": [
        "technology",
        "discovery",
        "nuclei",
        "passive"
    ],
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
    if target not in ("/reports/api/v2.0/System", "/reports/browse"):
        return []

    try:
        body = base64.b64decode(response.body).decode("latin-1").lower()
    except Exception:
        body = ""

    status = re.match(r"\s*(\d+)", response.status)
    signatures = (
        "power bi report server",
        "powerbireportserver",
        "<title>report server web portal</title>",
    )
    detected = (
        status is not None
        and int(status.group(1)) == 200
        and any(value in body for value in signatures)
    )
    return [finding("powerbi-report-server-detect", extracted=[])] if detected else []
