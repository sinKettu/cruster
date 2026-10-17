metadata = {
    "id": "limesurvey-detect",
    "about": "LimeSurvey Survey Software - Detect (passive Nuclei technology fingerprint)",
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
    if not status or int(status.group(1)) != 200:
        return []
    body = base64.b64decode(response.body).decode("latin-1").lower()
    markers = (
        'content="limesurvey',
        'alt="limesurvey survey software',
        "data-limesurvey-lang=",
        'alt="powered by limesurvey',
    )
    if any(marker in body for marker in markers):
        return [finding("limesurvey-detect")]
    return []
