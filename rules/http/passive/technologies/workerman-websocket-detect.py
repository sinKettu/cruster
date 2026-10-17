metadata = {
    "id": "workerman-websocket-detect",
    "about": "Workerman Websocket Detection (passive Nuclei technology fingerprint)",
    "authors": ["ProjectDiscovery contributors", "Cruster"],
    "references": [
        "https://github.com/projectdiscovery/nuclei-templates/tree/main/http/technologies"
    ],
    "tags": ["technology", "discovery", "nuclei", "passive"],
    "severity": "info",
    "type": "passive"
}

import base64
from cruster import HttpRequest, HttpResponse, finding


def scan(_request: HttpRequest, response: HttpResponse):
    servers = []
    for item in response.headers:
        if item["name"].lower() != "server":
            continue
        try:
            servers.append(base64.b64decode(item["value"]).decode("latin-1"))
        except Exception:
            continue
    if not any("workerman/" in server for server in servers):
        return []
    return [finding("workerman-websocket-detect", extracted=servers)]
