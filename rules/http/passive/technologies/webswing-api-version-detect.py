metadata = {
    "id": "webswing-api-version-detect",
    "about": "WebSwing REST API Version - Detection (passive Nuclei technology fingerprint)",
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
    if not request.uri.split("?", 1)[0].endswith("/rest/version"):
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []
    servers = []
    for item in response.headers:
        if item["name"].lower() != "server":
            continue
        try:
            servers.append(base64.b64decode(item["value"]).decode("latin-1"))
        except Exception:
            continue
    if not response.status.startswith("200") or not any(
        "webswing.org" in server for server in servers
    ):
        return []
    extracted = [match.group(1) for match in re.finditer(r"^(\d+(?:\.\d+)*)$", body)]
    return [finding("webswing-api-version-detect", extracted=extracted)]
