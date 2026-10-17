metadata = {
    "id": "tibco-businessconnect-detect",
    "about": "TIBCO BusinessConnect - Detect (passive Nuclei technology fingerprint)",
    "authors": ["ProjectDiscovery contributors", "Cruster"],
    "references": [
        "https://github.com/projectdiscovery/nuclei-templates/tree/main/http/technologies"
    ],
    "tags": ["technology", "discovery", "nuclei", "passive"],
    "severity": "info",
    "type": "passive",
}

import base64

from cruster import HttpRequest, HttpResponse, finding


def scan(_request: HttpRequest, response: HttpResponse):
    if response.status.lstrip().split(" ", 1)[0] != "200":
        return []
    headers = []
    for header in response.headers:
        try:
            value = base64.b64decode(header["value"]).decode("latin-1")
        except Exception:
            value = ""
        headers.append(header["name"] + ": " + value)
    if "TIBCO BusinessConnect" not in "\n".join(headers):
        return []
    return [finding("tibco-businessconnect-detect")]
