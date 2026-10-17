metadata = {
    "id": "default-redhat-test-page",
    "about": "Red Hat Enterprise Linux Test Page (passive Nuclei technology fingerprint)",
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
    marker = "<title>Test Page for the Apache HTTP Server on Red Hat Enterprise Linux</title>"
    return [finding("default-redhat-test-page")] if marker in body else []
