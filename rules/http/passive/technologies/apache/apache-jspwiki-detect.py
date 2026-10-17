metadata = {
    "id": "apache-jspwiki-detect",
    "about": "Apache JSPWiki - Detect (passive Nuclei technology fingerprint)",
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
    if not response.status.startswith("200"):
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""

    title_matches = "JSPWiki: Main" in body or "JSPWiki: Login" in body
    if not title_matches or "/jspwiki" not in body:
        return []

    match = re.search(r"JSPWiki v[0-9]+\.[0-9]+\.[0-9]+", body)
    return [
        finding("apache-jspwiki-detect", extracted=[match.group(0)] if match else [])
    ]
