metadata = {
    "id": "b2b-builder-detect",
    "about": "B2BBuilder Detect (passive Nuclei technology fingerprint)",
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


def _decode(value):
    try:
        return base64.b64decode(value).decode("latin-1")
    except Exception:
        return ""


def _status(value):
    match = re.match(r"\s*(\d+)", value)
    return int(match.group(1)) if match else 0


def scan(request: HttpRequest, response: HttpResponse):
    body = _decode(response.body)
    if _status(response.status) == 200 and (
        'content="B2Bbuilder"' in body or 'translateButtonId = "B2Bbuilder"' in body
    ):
        return [finding("b2b-builder-detect")]
    return []
