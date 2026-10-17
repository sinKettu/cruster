metadata = {
    "id": "interactsh-server",
    "about": "Interactsh Server (passive Nuclei technology fingerprint)",
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
    del request
    status = re.match(r"\s*(\d+)", response.status)
    if not status or int(status.group(1)) != 200:
        return []

    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    if "Interactsh Server" not in body:
        return []

    extracted = []
    for pattern in (r"<b>(.*)<\/b> server", r"from <b>(.*)<\/b>"):
        extracted.extend(match.group(1) for match in re.finditer(pattern, body))
    return [finding("interactsh-server", extracted=list(dict.fromkeys(extracted)))]
