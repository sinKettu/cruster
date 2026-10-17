metadata = {
    "id": "openarchives-detect",
    "about": "openarchives - Detection (passive Nuclei technology fingerprint)",
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
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    status = re.match(r"\s*(\d+)", response.status)
    markers = ("OAI-PMH", "www.openarchives.org", "</OAI-PMH")
    if (
        status
        and int(status.group(1)) == 200
        and all(marker in body for marker in markers)
    ):
        return [finding("openarchives-detect", extracted=[])]
    return []
