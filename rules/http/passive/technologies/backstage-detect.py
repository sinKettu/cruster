metadata = {
    "id": "backstage-detect",
    "about": "Backstage - Detect (passive Nuclei technology fingerprint)",
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
    status = re.match(r"\s*(\d+)", response.status)
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []

    detected = (
        status
        and int(status.group(1)) == 200
        and 'content="Backstage is an open platform for building developer portals"'
        in body
        and "module-backstage." in body
    )
    return [finding("backstage-detect")] if detected else []
