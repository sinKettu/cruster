metadata = {
    "id": "bagisto-detect",
    "about": "Bagisto - Detect (passive Nuclei technology fingerprint)",
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
    if (
        _status(response.status) == 200
        and re.search(r'<meta\s+name="generator"\s+content="Bagisto"', body)
        and "<!-- Built With Bagisto -->" in body
    ):
        return [finding("bagisto-detect")]
    return []
