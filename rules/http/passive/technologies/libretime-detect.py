metadata = {
    "id": "libretime-detect",
    "about": "LibreTime - Detect (passive Nuclei technology fingerprint)",
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
    has_title = re.search(r"<title>[^<]*LibreTime</title>", body) is not None
    has_assets = (
        "/css/radio-page/radio-page.css" in body and "/embed/weekly-program" in body
    )
    if has_title and has_assets:
        return [finding("libretime-detect")]
    return []
