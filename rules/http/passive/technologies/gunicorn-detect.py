metadata = {
    "id": "gunicorn-detect",
    "about": "Detect Gunicorn Server (passive Nuclei technology fingerprint)",
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
    del request
    server_values = []
    matched = False

    for header in response.headers:
        try:
            value = base64.b64decode(header["value"]).decode("latin-1")
        except Exception:
            continue

        header_line = header["name"] + ": " + value
        if re.search(r"gunicorn+", header_line):
            matched = True

        if header["name"].lower() == "server":
            server_values.append(value)

    if not matched:
        return []

    return [finding("gunicorn-detect", extracted=server_values)]
