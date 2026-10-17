metadata = {
    "id": "boa-web-server",
    "about": "Boa Web Server - Detect (passive Nuclei technology fingerprint)",
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
    headers = []
    servers = []
    for item in response.headers:
        try:
            value = base64.b64decode(item["value"]).decode("latin-1")
        except Exception:
            value = ""
        headers.append(item["name"] + ": " + value)
        if item["name"].lower() == "server":
            servers.append(value)

    header = "\n".join(headers)
    if re.search(r"Server: Boa/", header) is None:
        return []
    return [finding("boa-web-server", extracted=servers)]
