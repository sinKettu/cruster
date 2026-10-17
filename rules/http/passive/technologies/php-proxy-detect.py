metadata = {
    "id": "php-proxy-detect",
    "about": "PHP Proxy Detect (passive Nuclei technology fingerprint)",
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
    status = re.match(r"\s*(\d+)", response.status)
    versions = [
        match.group(1)
        for match in re.finditer(r'<meta name="version" content="([0-9.]+)">', body)
    ]
    detected = (
        status is not None
        and int(status.group(1)) == 200
        and "<title>PHP-Proxy</title>" in body
    )
    return [finding("php-proxy-detect", extracted=versions)] if detected else []
