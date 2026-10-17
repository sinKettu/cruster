metadata = {
    "id": "default-detect-generic",
    "about": "Default Web Application Detection (passive Nuclei technology fingerprint)",
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
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    markers = ("<title>Default</title>", "<title>Welcome to</title>")
    return (
        [finding("default-detect-generic")]
        if any(marker in body for marker in markers)
        else []
    )
