metadata = {
    "id": "librebooking-detect",
    "about": "LibreBooking - Detect (passive Nuclei technology fingerprint)",
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
    if not status or int(status.group(1)) != 200:
        return []
    body = base64.b64decode(response.body).decode("latin-1")
    markers = (
        "<title>LibreBooking - Log In</title>",
        "librebooking.css",
        "img/librebooking.png",
    )
    if all(marker in body for marker in markers):
        return [finding("librebooking-detect")]
    return []
