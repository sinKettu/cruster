metadata = {
    "id": "tinyproxy-detect",
    "about": "Tinyproxy - Detect (passive Nuclei technology fingerprint)",
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
    headers = []
    for header in response.headers:
        try:
            value = base64.b64decode(header["value"]).decode("latin-1")
        except Exception:
            value = ""
        headers.append(header["name"] + ": " + value)
        if header["name"].lower() == "server":
            servers.append(value)
    if "server: tinyproxy" not in "\n".join(headers).lower():
        return []
    return [finding("tinyproxy-detect", extracted=servers)]
