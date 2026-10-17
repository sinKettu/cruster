metadata = {
    "id": "affine-detect",
    "about": "AFFiNE - Detect (passive Nuclei technology fingerprint)",
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

    markers = (
        "<title>AFFiNE</title>",
        'name="env:isSelfHosted" content="true"',
        "npm-blocksuite",
    )
    if not all(marker in body for marker in markers):
        return []

    return [finding("affine-detect")]
