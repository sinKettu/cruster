metadata = {
    "id": "wordpress-eol",
    "about": "WordPress End-of-Life - Detect (passive Nuclei technology fingerprint)",
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
    body = base64.b64decode(response.body).decode("latin-1")
    patterns = (
        r'(?i)<meta[^>]*name=["\']?generator["\']?[^>]*content=["\']?WordPress\s+([0-9.]+)',
        r"wp-includes/js/wp-emoji-release(?:\.min)?\.js\?ver=([0-9]+\.[0-9.]+)",
        r"wp-includes/css/dist/block-library/style(?:\.min)?\.css\?ver=([0-9]+\.[0-9.]+)",
        r"wp-includes/blocks/[a-z-]+/style(?:\.min)?\.css\?ver=([0-9]+\.[0-9.]+)",
    )
    versions = []
    for pattern in patterns:
        versions.extend(match.group(1) for match in re.finditer(pattern, body))
    identified = any(
        marker in body.lower()
        for marker in ("/wp-content/", "/wp-includes/", "/wp-json/")
    )
    if versions and identified and tuple(map(int, versions[0].split("."))) < (6, 9):
        return [finding("wordpress-eol", extracted=list(dict.fromkeys(versions)))]
    return []
