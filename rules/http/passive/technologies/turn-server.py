metadata = {
    "id": "turn-server",
    "about": "TURN Server (passive Nuclei technology fingerprint)",
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
    if response.status.lstrip().split(" ", 1)[0] != "200":
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []
    if "<title>TURN Server</title>" not in body:
        return []

    servers = []
    for item in response.headers:
        if item["name"].lower() == "server":
            try:
                servers.append(base64.b64decode(item["value"]).decode("latin-1"))
            except Exception:
                continue
    return [finding("turn-server", extracted=servers)]
