metadata = {
    "id": "logto-detect",
    "about": "Logto - Detect (passive Nuclei technology fingerprint)",
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
    if not status or int(status.group(1)) not in (200, 302):
        return []
    body = base64.b64decode(response.body).decode("latin-1")
    headers = "\n".join(
        header["name"] + ": " + base64.b64decode(header["value"]).decode("latin-1")
        for header in response.headers
    ).lower()
    if "window.logtoSsr" in body or "logto-core-request-id:" in headers:
        return [finding("logto-detect")]
    return []
