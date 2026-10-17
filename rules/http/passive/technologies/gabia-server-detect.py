metadata = {
    "id": "gabia-server-detect",
    "about": "Gabia Server - Detection (passive Nuclei technology fingerprint)",
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
    del request
    status = re.match(r"\s*(\d+)", response.status)
    if not status or int(status.group(1)) != 200:
        return []
    servers = []
    for header in response.headers:
        if header["name"].lower() != "server":
            continue
        try:
            servers.append(base64.b64decode(header["value"]).decode("latin-1"))
        except Exception:
            pass
    return (
        [finding("gabia-server-detect", extracted=servers)]
        if any("gabia" in value for value in servers)
        else []
    )
