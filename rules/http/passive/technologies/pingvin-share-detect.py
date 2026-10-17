metadata = {
    "id": "pingvin-share-detect",
    "about": "Pingvin Share - Detect (passive Nuclei technology fingerprint)",
    "authors": ["ProjectDiscovery contributors", "Cruster"],
    "references": [
        "https://github.com/projectdiscovery/nuclei-templates/tree/main/http/technologies"
    ],
    "tags": ["technology", "discovery", "nuclei", "passive"],
    "severity": "info",
    "type": "passive"
}

import base64
import re
from cruster import HttpRequest, HttpResponse, finding



def scan(request: HttpRequest, response: HttpResponse):
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    status = re.match(r"\s*(\d+)", response.status)
    title = re.search(r"<title>[^<]*Pingvin Share[^<]*</title>", body)
    detected = (
        status is not None
        and int(status.group(1)) == 200
        and title is not None
        and 'content="An open-source and self-hosted sharing platform."' in body
    )
    return [finding("pingvin-share-detect", extracted=[])] if detected else []
