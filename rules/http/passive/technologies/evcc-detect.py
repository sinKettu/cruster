metadata = {
    "id": "evcc-detect",
    "about": "EVCC - Detect (passive Nuclei technology fingerprint)",
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
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    required = (
        "<title>evcc</title>",
        'content="evcc - Sonne tanken',
        'name="application-name" content="evcc"',
    )
    return (
        [finding("evcc-detect")]
        if status
        and int(status.group(1)) == 200
        and all(value in body for value in required)
        else []
    )
