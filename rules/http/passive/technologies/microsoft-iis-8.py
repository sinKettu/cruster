metadata = {
    "id": "microsoft-iis-8",
    "about": "Microsoft Internet Information Services 8 (passive Nuclei technology fingerprint)",
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
    body = base64.b64decode(response.body).decode("latin-1")
    status = re.match(r"\s*(\d+)", response.status)
    if (
        status
        and int(status.group(1)) == 200
        and "<title>Microsoft Internet Information Services 8</title>" in body
    ):
        return [finding("microsoft-iis-8")]
    return []
