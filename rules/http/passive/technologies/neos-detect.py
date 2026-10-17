metadata = {
    "id": "neos-detect",
    "about": "Neos CMS detection (passive Nuclei technology fingerprint)",
    "authors": ["ProjectDiscovery contributors", "Cruster"],
    "references": [
        "https://github.com/projectdiscovery/nuclei-templates/tree/main/http/technologies"
    ],
    "tags": ["technology", "discovery", "nuclei", "passive"],
    "severity": "info",
    "type": "passive"
}
import base64

from cruster import HttpRequest, HttpResponse, finding


def scan(request: HttpRequest, response: HttpResponse):
    del request
    body = base64.b64decode(response.body).decode("latin-1")
    powered_by = []
    for item in response.headers:
        if item["name"].lower().replace("-", "_") == "x_flow_powered":
            value = base64.b64decode(item["value"]).decode("latin-1")
            powered_by.extend(value.split("\n"))

    if "This website is powered by Neos" in body or powered_by:
        return [finding("neos-detect", extracted=powered_by)]
    return []
