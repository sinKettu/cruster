metadata = {
    "id": "firstpromoter-detect",
    "about": "FirstPromoter - Detect (passive Nuclei technology fingerprint)",
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
    if status is None or int(status.group(1)) != 200:
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []
    markers = (
        "cdn.firstpromoter.com/fpr.js",
        "cdn.firstpromoter.com/fprom.js",
        "//cdn.firstpromoter.com",
    )
    return (
        [finding("firstpromoter-detect")]
        if any(marker in body for marker in markers)
        else []
    )
