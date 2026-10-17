metadata = {
    "id": "mitel-version-detect",
    "about": "Mitel MiCollab Unified Communications Server (UCS) - Detect (passive Nuclei technology fingerprint)",
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
    target = target.split("#", 1)[0].split("?", 1)[0] or "/"
    if target != "/ucs/micollab/version.json":
        return []
    body = base64.b64decode(response.body).decode("latin-1")
    status = re.match(r"\s*(\d+)", response.status)
    if not status or int(status.group(1)) != 200 or '{"version":' not in body:
        return []
    # The source JSON extractor did not produce public evidence in the
    # generated interpreter, so detection intentionally has no extraction.
    return [finding("mitel-version-detect")]
