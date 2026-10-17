metadata = {
    "id": "deskpro-default-page",
    "about": "Deskpro Private Controller - Default Page Detect (passive Nuclei technology fingerprint)",
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
    status = re.match(r"\s*(\d+)", response.status)
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    markers = ("<title>Deskpro Private Controller</title>", "DPC Default Page")
    detected = (
        status
        and int(status.group(1)) == 200
        and any(marker in body for marker in markers)
    )
    return [finding("deskpro-default-page")] if detected else []
