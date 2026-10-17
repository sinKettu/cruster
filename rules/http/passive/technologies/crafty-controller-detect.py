metadata = {
    "id": "crafty-controller-detect",
    "about": "Crafty Controller - Detect (passive Nuclei technology fingerprint)",
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
    status = re.match(r"\s*(\d+)", response.status)
    if not status or int(status.group(1)) != 200:
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    markers = (
        "<title>Crafty Controller</title>",
        "/static/assets/crafty.webmanifest",
        "/static/assets/css/crafty.css",
    )
    return (
        [finding("crafty-controller-detect")]
        if all(marker in body for marker in markers)
        else []
    )
