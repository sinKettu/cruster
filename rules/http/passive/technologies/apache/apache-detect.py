metadata = {
    "id": "apache-detect",
    "about": "Apache Detection (passive Nuclei technology fingerprint)",
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
    headers = [
        (item["name"], base64.b64decode(item["value"]).decode("latin-1"))
        for item in response.headers
    ]
    text = "\n".join(name + ": " + value for name, value in headers)
    if not response.status.startswith("200") or re.search(r"Apache+", text) is None:
        return []

    servers = [value for name, value in headers if name.lower() == "server"]
    return [finding("apache-detect", extracted=servers)]
