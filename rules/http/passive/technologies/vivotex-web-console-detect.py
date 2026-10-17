metadata = {
    "id": "vivotex-web-console-detect",
    "about": "VIVOTEK Web Console Detect (passive Nuclei technology fingerprint)",
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
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []
    header_values = []
    for item in response.headers:
        try:
            header_values.append(base64.b64decode(item["value"]).decode("latin-1"))
        except Exception:
            continue
    if not response.status.startswith("200"):
        return []
    if "<title>VIVOTEK Web Console</title>" not in body:
        return []
    if not any("text/html" in value for value in header_values):
        return []
    return [finding("vivotex-web-console-detect")]
