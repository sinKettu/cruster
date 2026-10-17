metadata = {
    "id": "phplist-detect",
    "about": "phpList - Detect (passive Nuclei technology fingerprint)",
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
        match.group(1) for match in re.finditer(r"(?i)version.((\d\.?)+)", body)
    ]
    lower_body = body.lower()
    detected = any(
        value.lower() in lower_body
        for value in ('content="phpList', "phpList Ltd", "phpList</a>")
    )
    return [finding("phplist-detect", extracted=versions)] if detected else []
