metadata = {
    "id": "thingspanel-detect",
    "about": "ThingsPanel - Detect (passive Nuclei technology fingerprint)",
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
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []
    indicators = ("<title>ThingsPanel</title>", "clarity', 'script', 'no47mbcicr'")
    if not all(indicator in body for indicator in indicators):
        return []
    return [finding("thingspanel-detect")]
