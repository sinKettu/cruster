metadata = {
    "id": "default-symfony-page",
    "about": "Symfony Default Page (passive Nuclei technology fingerprint)",
    "authors": ["ProjectDiscovery contributors", "Cruster"],
    "references": [
        "https://github.com/projectdiscovery/nuclei-templates/tree/main/http/technologies"
    ],
    "tags": ["technology", "discovery", "nuclei", "passive"],
    "severity": "info",
    "type": "passive",
}
import base64
import re

from cruster import HttpRequest, HttpResponse, finding


def scan(request: HttpRequest, response: HttpResponse):
    del request
    status = re.match(r"\s*(\d+)", response.status)
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    detected = (
        status
        and int(status.group(1)) == 404
        and "Welcome to Symfony!" in body
        and "Documentation" in body
    )
    if not detected:
        return []
    version = re.search(r"([0-9.]+)</span>", body)
    return [
        finding("default-symfony-page", extracted=[version.group(1)] if version else [])
    ]
