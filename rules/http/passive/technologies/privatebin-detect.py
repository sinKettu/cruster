metadata = {
    "id": "privatebin-detect",
    "about": "PrivateBin - Detect (passive Nuclei technology fingerprint)",
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
    versions = [
        match.group(1)
        for match in re.finditer(r"privatebin\.[a-z]{2,3}\?([0-9.]+)", body)
    ]
    status = re.match(r"\s*(\d+)", response.status)
    lower_body = body.lower()
    detected = (
        status is not None
        and int(status.group(1)) == 200
        and any(
            value in lower_body
            for value in ("privatebin.js", "privatebin</title>", 'content="privatebin')
        )
    )
    return [finding("privatebin-detect", extracted=versions)] if detected else []
