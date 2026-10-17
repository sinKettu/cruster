metadata = {
    "id": "sailsjs-detect",
    "about": "Sails.js Framework - Detect (passive Nuclei technology fingerprint)",
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
    header_lines = []
    for header in response.headers:
        try:
            value = base64.b64decode(header["value"]).decode("latin-1")
        except Exception:
            value = ""
        header_lines.append(header["name"] + ": " + value)

    headers = "\n".join(header_lines)
    if (
        "x-powered-by: sails" not in headers.lower()
        or re.search(r"(?i)set-cookie:\s*sails\.sid=", headers) is None
    ):
        return []

    return [finding("sailsjs-detect", extracted=[])]
