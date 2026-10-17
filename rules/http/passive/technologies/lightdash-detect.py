metadata = {
    "id": "lightdash-detect",
    "about": "Lightdash - Detect (passive Nuclei technology fingerprint)",
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
    if not status or int(status.group(1)) != 200:
        return []
    body = base64.b64decode(response.body).decode("latin-1")
    body_markers = (
        "<title>Lightdash</title>",
        'content="Lightdash - Explore and visualize your team\'s analytics and metrics."',
    )
    if not all(marker in body for marker in body_markers):
        return []
    headers = "\n".join(
        header["name"] + ": " + base64.b64decode(header["value"]).decode("latin-1")
        for header in response.headers
    )
    if "lightdash-version:" in headers.lower():
        return [finding("lightdash-detect")]
    return []
