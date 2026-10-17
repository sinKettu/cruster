metadata = {
    "id": "uptrace-detect",
    "about": "Uptrace - Detect (passive Nuclei technology fingerprint)",
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
    if request.uri.split("?", 1)[0].rstrip("/").endswith(":"):
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []

    asset_markers = ["/assets/vuetify-", "/js/chunk-vendors."]
    title_pattern = r"(?i)<title>Uptrace(?:: Open Source APM)?</title>"
    if not response.status.startswith("200"):
        return []
    if re.search(title_pattern, body) is None:
        return []
    if not any(marker in body for marker in asset_markers):
        return []
    return [finding("uptrace-detect")]
