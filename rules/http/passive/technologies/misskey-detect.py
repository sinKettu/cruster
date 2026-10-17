metadata = {
    "id": "misskey-detect",
    "about": "Misskey - Detect (passive Nuclei technology fingerprint)",
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
    body = base64.b64decode(response.body).decode("latin-1")
    status = re.match(r"\s*(\d+)", response.status)
    indicators = (
        "Thank you for using Misskey!",
        "github.com/misskey-dev/misskey",
        'name="application-name" content="Misskey"',
    )
    if (
        status
        and int(status.group(1)) == 200
        and all(value in body for value in indicators)
    ):
        return [finding("misskey-detect")]
    return []
