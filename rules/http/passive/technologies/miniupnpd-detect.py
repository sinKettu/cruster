metadata = {
    "id": "miniupnpd-detect",
    "about": "MiniUPnPd - Detect (passive Nuclei technology fingerprint)",
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
    headers = "\n".join(
        item["name"] + ": " + base64.b64decode(item["value"]).decode("latin-1")
        for item in response.headers
    )
    if "MiniUPnPd" not in headers:
        return []
    versions = [
        match.group(1) for match in re.finditer(r"MiniUPnPd/([0-9.]+)", headers)
    ]
    return [finding("miniupnpd-detect", extracted=versions)]
