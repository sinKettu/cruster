metadata = {
    "id": "lighttpd-detect",
    "about": "Lighttpd Web Server - Detect (passive Nuclei technology fingerprint)",
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
    headers = "\n".join(
        header["name"] + ": " + base64.b64decode(header["value"]).decode("latin-1")
        for header in response.headers
    )
    if "lighttpd" not in headers:
        return []
    versions = [match.group(1) for match in re.finditer(r"lighttpd/([0-9.]+)", headers)]
    return [finding("lighttpd-detect", extracted=versions)]
