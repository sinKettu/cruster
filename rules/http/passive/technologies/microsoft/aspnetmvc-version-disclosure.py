metadata = {
    "id": "aspnetmvc-version-disclosure",
    "about": "AspNetMvc Version - Detect (passive Nuclei technology fingerprint)",
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
    values = []
    for h in response.headers:
        v = base64.b64decode(h["value"]).decode("latin-1")
        if h["name"].lower().replace("-", "_") == "x_aspnetmvc_version":
            values.append(v)
    return [finding("aspnetmvc-version-disclosure", extracted=values)] if values else []
