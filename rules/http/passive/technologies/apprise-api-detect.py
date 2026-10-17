metadata = {
    "id": "apprise-api-detect",
    "about": "Apprise API - Detect (passive Nuclei technology fingerprint)",
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
    target = request.uri.split("://", 1)[-1]
    target = target[target.find("/") :] if "/" in target else "/"
    target = target.split("?", 1)[0].split("#", 1)[0]
    status = re.match(r"\s*(\d+)", response.status)
    if target != "/" or status is None or int(status.group(1)) != 200:
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []

    title_matches = re.search(r"(?i)<title>\s*Apprise API\s*</title>", body) is not None
    if not title_matches or "/s/favicon.ico" not in body or "Apprise Logo" not in body:
        return []

    return [finding("apprise-api-detect")]
