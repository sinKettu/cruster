import base64
import re
from cruster import HttpRequest, HttpResponse, finding

metadata = {
    "id": "pingaccess-detect",
    "about": "PingAccess - Detect (passive Nuclei technology fingerprint)",
    "authors": ["ProjectDiscovery contributors", "Cruster"],
    "references": [
        "https://github.com/projectdiscovery/nuclei-templates/tree/main/http/technologies"
    ],
    "tags": ["technology", "discovery", "nuclei", "passive"],
    "severity": "info",
    "type": "passive",
}


def scan(request: HttpRequest, response: HttpResponse):
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    status = re.match(r"\s*(\d+)", response.status)
    detected = (
        status is not None
        and int(status.group(1)) == 403
        and "/pingaccess/assets/css/main.css" in body
        and "/pingaccess/assets/images/favicon.ico" in body
    )
    return [finding("pingaccess-detect", extracted=[])] if detected else []
