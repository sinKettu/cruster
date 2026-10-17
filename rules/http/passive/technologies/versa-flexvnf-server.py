metadata = {
    "id": "versa-flexvnf-server",
    "about": "Versa FlexVNF Server (passive Nuclei technology fingerprint)",
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


def scan(request: HttpRequest, response: HttpResponse):
    if not request.uri.split("?", 1)[0].endswith("/&"):
        return []
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
    header_text = "\n".join(headers)
    if not response.status.startswith("200"):
        return []
    if "Server: Versa" not in header_text or "FlexVNF" not in header_text:
        return []
    return [finding("versa-flexvnf-server", extracted=servers)]
