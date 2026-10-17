metadata = {
    "id": "versa-analytics-server",
    "about": "Versa Analytics Server Detection (passive Nuclei technology fingerprint)",
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


def scan(request: HttpRequest, response: HttpResponse):
    del request
    for item in response.headers:
        if item["name"].lower() != "server":
            continue
        value = base64.b64decode(item["value"]).decode("latin-1")
        if "Versa-Analytics-Server" in value:
            return [finding("versa-analytics-server", extracted=[value])]
    return []
