metadata = {
    "id": "hotjar-rum-detect",
    "about": "Hotjar RUM - Detect (passive Nuclei technology fingerprint)",
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
    status_match = re.match(r"\s*(\d+)", response.status)
    if status_match is None or int(status_match.group(1)) not in (200, 400):
        return []

    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []

    patterns = (
        r"h\._hjSettings\s*=\s*\{\s*hjid\s*:",
        r"_hjSettings\s*=\s*\{\s*hjid\s*:",
        r"<script[^>]*src=['\"][^'\"]*static\.hotjar\.com[^'\"]*['\"]",
        r"<script[^>]*src=['\"][^'\"]*script\.hotjar\.com[^'\"]*['\"]",
        r"window\.hj\s*=\s*window\.hj\s*\|\|",
        r"\(function\s*\([^)]*\)\s*\{[^}]*hotjar[^}]*hjid",
    )
    if not any(re.search(pattern, body) for pattern in patterns):
        return []

    return [finding("hotjar-rum-detect")]
