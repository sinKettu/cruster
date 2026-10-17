metadata = {
    "id": "cinny-detect",
    "about": "Cinny - Detect (passive Nuclei technology fingerprint)",
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
    status = re.match(r"\s*(\d+)", response.status)
    if not status or int(status.group(1)) != 200:
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    if not all(
        value in body
        for value in (
            "<title>Cinny</title>",
            "A Matrix client where you can enjoy the conversation",
            'name="application-name" content="Cinny"',
        )
    ):
        return []
    return [finding("cinny-detect")]
