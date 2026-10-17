metadata = {
    "id": "vouch-proxy-detect",
    "about": "Vouch Proxy - Detect (passive Nuclei technology fingerprint)",
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
    headers = []
    for item in response.headers:
        try:
            value = base64.b64decode(item["value"]).decode("latin-1")
        except Exception:
            value = ""
        headers.append(item["name"] + ": " + value)
    header_text = "\n".join(headers)
    markers = ["vouch-failcount=", "X-Vouch-Token="]
    if not response.status.startswith("302"):
        return []
    if not all(marker in header_text for marker in markers):
        return []
    return [finding("vouch-proxy-detect")]
