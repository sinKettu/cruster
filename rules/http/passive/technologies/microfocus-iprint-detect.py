metadata = {
    "id": "microfocus-iprint-detect",
    "about": "Micro Focus iPrint Appliance - Detect (passive Nuclei technology fingerprint)",
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
    target = request.uri.split("://", 1)[-1]
    target = target[target.find("/") :] if "/" in target else "/"
    target = target.split("#", 1)[0].split("?", 1)[0] or "/"
    if target != "/welcome/":
        return []
    body = base64.b64decode(response.body).decode("latin-1")
    status = re.match(r"\s*(\d+)", response.status)
    indicators = (
        "micro focus iprint appliance",
        "/ippdocs/clients/",
        "micro focus iprint",
        "iprint appliance",
    )
    if (
        not status
        or int(status.group(1)) != 200
        or not any(value in body.lower() for value in indicators)
    ):
        return []
    versions = [
        match.group(1)
        for match in re.finditer(r"(?i)/welcome/css/core([0-9]+).css", body)
    ]
    return [finding("microfocus-iprint-detect", extracted=versions)]
