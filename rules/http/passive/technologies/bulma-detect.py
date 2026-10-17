metadata = {
    "id": "bulma-detect",
    "about": "Bulma CSS Framework - Detect (passive Nuclei technology fingerprint)",
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

    if not re.search(r'(?i)(/|")bulma(\.min)?\.css', body):
        return []

    extracted = []
    for pattern in (
        r"(?i)bulma(?:@|/)([0-9.]+)/",
        r"(?i)bulma-([0-9.]+)\.css",
    ):
        extracted.extend(match.group(1) for match in re.finditer(pattern, body))
    return [finding("bulma-detect", extracted=list(dict.fromkeys(extracted)))]
