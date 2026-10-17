metadata = {
    "id": "statamic-detect",
    "about": "Statamic - Detect (passive Nuclei technology fingerprint)",
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
    header_lines = []
    for header in response.headers:
        try:
            value = base64.b64decode(header["value"]).decode("latin-1")
        except Exception:
            value = ""
        header_lines.append(header["name"] + ": " + value)

    headers = "\n".join(header_lines)
    if re.search(r"X-Powered-By:(.*)Statamic", headers) is None:
        return []

    return [finding("statamic-detect", extracted=[])]
