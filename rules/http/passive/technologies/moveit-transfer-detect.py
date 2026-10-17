metadata = {
    "id": "moveit-transfer-detect",
    "about": "Moveit File Transfer - Detect (passive Nuclei technology fingerprint)",
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
    body = base64.b64decode(response.body).decode("latin-1").lower()
    status = re.match(r"\s*(\d+)", response.status)
    indicators = (
        "stylesheet_moveit",
        "moveit.transfer",
        "moveitpopup",
        "moveit automation",
    )
    if (
        status
        and int(status.group(1)) == 200
        and any(value in body for value in indicators)
    ):
        return [finding("moveit-transfer-detect")]
    return []
