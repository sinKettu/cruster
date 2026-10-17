metadata = {
    "id": "iobroker-detect",
    "about": "ioBroker - Detect (passive Nuclei technology fingerprint)",
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
    status = re.match(r"\s*(\d+)", response.status)
    if not status or int(status.group(1)) != 200:
        return []

    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""

    titles = (
        "<title>Admin</title>",
        "<title>Web Login (ioBroker)</title>",
        "<title>ioBroker</title>",
    )
    if "ioBroker" not in body or not any(title in body for title in titles):
        return []

    return [finding("iobroker-detect")]
