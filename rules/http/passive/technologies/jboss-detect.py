metadata = {
    "id": "jboss-detect",
    "about": "JBoss detected (passive Nuclei technology fingerprint)",
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
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""

    if (
        "JBoss EAP 7" not in body
        and re.search(r"<title>Welcome to JBoss(.*)</title>", body) is None
    ):
        return []

    extracted = [
        match.group(1) for match in re.finditer(r"<h1>Welcome to (.+)</h1>", body)
    ]
    return [finding("jboss-detect", extracted=extracted)]
