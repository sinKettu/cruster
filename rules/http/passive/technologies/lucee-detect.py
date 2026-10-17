metadata = {
    "id": "lucee-detect",
    "about": "Detect Lucee (passive Nuclei technology fingerprint)",
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
    body = base64.b64decode(response.body).decode("latin-1")
    headers = "\n".join(
        header["name"] + ": " + base64.b64decode(header["value"]).decode("latin-1")
        for header in response.headers
    )
    patterns = (
        r"(?i)X-Lucee-Version",
        r"(?i)X-CB-Server: LUCEE",
        r"(?i)X-IDG-Appserver: Lucee",
    )
    if (
        any(re.search(pattern, headers) for pattern in patterns)
        or "Lucee Function Reference" in body
    ):
        return [finding("lucee-detect")]
    return []
