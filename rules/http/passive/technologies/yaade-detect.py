metadata = {
    "id": "yaade-detect",
    "about": "Yaade - Detect (passive Nuclei technology fingerprint)",
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
        return []

    body_markers = (
        "<title>Yaade</title>",
        'rel="modulepreload" href="./assets/vendor.',
    )
    if not all(marker in body for marker in body_markers):
        return []

    cookie_marker = "set-cookie: vertx-web.session="
    for item in response.headers:
        try:
            value = base64.b64decode(item["value"]).decode("latin-1")
        except Exception:
            continue

        header_line = item["name"] + ": " + value
        if cookie_marker in header_line.lower():
            return [finding("yaade-detect")]

    return []
