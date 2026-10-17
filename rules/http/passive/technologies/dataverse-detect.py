metadata = {
    "id": "dataverse-detect",
    "about": "Dataverse - Detect (passive Nuclei technology fingerprint)",
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
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    title = re.search(r"(?i)<title>[^<]*Dataverse[^<]*</title>", body)
    resource = "/jakarta.faces.resource/" in body or "/javax.faces.resource/" in body
    return [finding("dataverse-detect")] if title and resource else []
