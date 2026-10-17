metadata = {
    "id": "metasfresh-detect",
    "about": "metasfresh - Detect (passive Nuclei technology fingerprint)",
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
        and re.search(r"(?i)<title>metasfresh</title>", body)
        and re.search(r"/bundle-[a-z0-9-]+\.js", body)
        and '<script src="/config.js"></script>' in body
    ):
        return [finding("metasfresh-detect")]
    return []
