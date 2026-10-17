metadata = {
    "id": "goliath-detect",
    "about": "Goliath Detection (passive Nuclei technology fingerprint)",
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
    servers = []
    matched = False
    for header in response.headers:
        try:
            value = base64.b64decode(header["value"]).decode("latin-1")
        except Exception:
            continue
        line = header["name"] + ": " + value
        if re.search(r"Goliath+", line):
            matched = True
        if header["name"].lower() == "server":
            servers.append(value)
    return [finding("goliath-detect", extracted=servers)] if matched else []
