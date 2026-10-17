metadata = {
    "id": "zincsearch-detect",
    "about": "ZincSearch - Detect (passive Nuclei technology fingerprint)",
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
    if "/" in target:
        target = target[target.find("/") :]
    else:
        target = "/"
    target = target.split("?", 1)[0].split("#", 1)[0]
    if target != "/":
        return []

    status = re.match(r"\s*(\d+)", response.status)
    if not status or int(status.group(1)) != 200:
        return []

    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []

    if "<title>ZincSearch</title>" not in body:
        return []

    asset_pattern = r"\./assets/index[.-][A-Za-z0-9_-]+\.js"
    if not re.search(asset_pattern, body):
        return []

    return [finding("zincsearch-detect")]
