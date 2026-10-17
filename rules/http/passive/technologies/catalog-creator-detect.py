metadata = {
    "id": "catalog-creator-detect",
    "about": "CATALOGcreator Page Login Panel - Detect (passive Nuclei technology fingerprint)",
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
    if not status or int(status.group(1)) != 200:
        return []

    try:
        body = base64.b64decode(response.body).decode("latin-1").lower()
    except Exception:
        body = ""
    if (
        "powered by catalogcreator" not in body
        and 'content="catalogcreator' not in body
    ):
        return []
    return [finding("catalog-creator-detect", extracted=[])]
