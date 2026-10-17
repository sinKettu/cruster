metadata = {
    "id": "flagsmith-detect",
    "about": "Flagsmith - Detect (passive Nuclei technology fingerprint)",
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
    if (
        re.search(
            r"<title>Flagsmith - Feature Flags, Feature Toggles and Remote Config[^<]*</title>",
            body,
        )
        is None
    ):
        return []
    markers = ('src="/config/project-overrides"', 'href="/static/images/favicon.ico"')
    return (
        [finding("flagsmith-detect")]
        if all(marker in body for marker in markers)
        else []
    )
