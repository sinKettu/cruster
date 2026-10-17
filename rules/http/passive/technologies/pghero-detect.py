import base64
import re

from cruster import HttpRequest, HttpResponse, finding

metadata = {
    "id": "pghero-detect",
    "about": "PgHero - Detect (passive Nuclei technology fingerprint)",
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
        body = base64.b64decode(response.body).decode("latin-1").lower()
    except Exception:
        body = ""
    status = re.match(r"\s*(\d+)", response.status)
    detected = (
        status is not None
        and int(status.group(1)) == 200
        and any(
            value in body
            for value in ("<title>pghero", "/assets/pghero/", ">pghero</a>")
        )
    )
    return [finding("pghero-detect", extracted=[])] if detected else []
