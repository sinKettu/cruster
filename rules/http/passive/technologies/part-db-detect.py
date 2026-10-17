import base64
import re
from cruster import HttpRequest, HttpResponse, finding

metadata = {
    "id": "part-db-detect",
    "about": "Part-DB - Detect (passive Nuclei technology fingerprint)",
    "authors": ["ProjectDiscovery contributors", "Cruster"],
    "references": [
        "https://github.com/projectdiscovery/nuclei-templates/tree/main/http/technologies",
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
    signatures = (
        "<title>Part-DB</title>",
        'content="Part-DB"',
        "https://github.com/Part-DB/Part-DB-server",
    )
    detected = (
        status is not None
        and int(status.group(1)) == 200
        and all(value in body for value in signatures)
    )
    return [finding("part-db-detect", extracted=[])] if detected else []
