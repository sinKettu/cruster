metadata = {
    "id": "celebrus-detect",
    "about": "Celebrus - Detect (passive Nuclei technology fingerprint)",
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
    status = re.match(r"\s*(\d+)", response.status)
    if not status or int(status.group(1)) != 200:
        return []

    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    lowered = body.lower()
    if (
        'content="celebrus technologies ltd' not in lowered
        and "<title>celebrus" not in lowered
    ):
        return []
    extracted = [
        match.group(1)
        for match in re.finditer(r"Celebrus\s+([0-9.]+)\s+Collection", body)
    ]
    return [finding("celebrus-detect", extracted=extracted)]
