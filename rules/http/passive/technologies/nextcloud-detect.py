metadata = {
    "id": "nextcloud-detect",
    "about": "Nextcloud Detect (passive Nuclei technology fingerprint)",
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
    body = base64.b64decode(response.body).decode("latin-1")
    status = re.match(r"\s*(\d+)", response.status)
    if not status or int(status.group(1)) != 200:
        return []
    if "var nc_lastLogin" not in body or "var nc_pageLoad" not in body:
        return []

    versions = re.findall(r'(?m)"version":"([0-9.]+)",', body)
    return [finding("nextcloud-detect", extracted=versions)]
