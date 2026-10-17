metadata = {
    "id": "openmetadata-detect",
    "about": "OpenMetadata - Detect (passive Nuclei technology fingerprint)",
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
    try:
        body = base64.b64decode(response.body).decode("latin-1").lower()
    except Exception:
        body = ""
    status = re.match(r"\s*(\d+)", response.status)
    markers = ("<title>openmetadata", 'content="openmetadata', "/openmetadata.bundle")
    if (
        status
        and int(status.group(1)) in (200, 404)
        and any(marker in body for marker in markers)
    ):
        return [finding("openmetadata-detect", extracted=[])]
    return []
