metadata = {
    "id": "smtp2go-detect",
    "about": "SMTP2GO Detect (passive Nuclei technology fingerprint)",
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
    status = re.match(r"\s*(\d+)", response.status)
    if status is None or int(status.group(1)) != 200:
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    signatures = (
        "<title>SMTP2GO</title>",
        "2022 &copy; SMTP2GO.",
        "mailto:ticket@smtp2go.com",
        'href="https://www.smtp2go.com/"',
    )
    if not any(signature in body for signature in signatures):
        return []
    return [finding("smtp2go-detect", extracted=[])]
