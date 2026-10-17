metadata = {
    "id": "default-lucee-page",
    "about": "Lucee Default Page (passive Nuclei technology fingerprint)",
    "authors": ["ProjectDiscovery contributors", "Cruster"],
    "references": [
        "https://github.com/projectdiscovery/nuclei-templates/tree/main/http/technologies"
    ],
    "tags": ["technology", "discovery", "nuclei", "passive"],
    "severity": "info",
    "type": "passive"
}
import base64
import re

from cruster import HttpRequest, HttpResponse, finding


def scan(request: HttpRequest, response: HttpResponse):
    del request
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    if "<title>Rapid web development with Lucee!</title>" not in body:
        return []
    version = re.search(r"Lucee ([0-9.\-A-Z]+) on your system", body)
    return [
        finding("default-lucee-page", extracted=[version.group(1)] if version else [])
    ]
