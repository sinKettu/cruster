metadata = {
    "id": "htmly-detect",
    "about": "HTMLy - Detect (passive Nuclei technology fingerprint)",
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
    status_match = re.match(r"\s*(\d+)", response.status)
    if status_match is None or int(status_match.group(1)) != 200:
        return []

    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []

    generator = r'<meta name="generator" content="HTMLy(?: v[0-9.]+)?"\s*/?>'
    if re.search(generator, body) is None:
        return []
    if re.search(r'href=["\'][^"\']*/feed/rss["\']', body) is None:
        return []

    return [finding("htmly-detect")]
