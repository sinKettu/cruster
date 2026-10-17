metadata = {
    "id": "jitsi-meet-detect",
    "about": "Jitsi Meet Page Detect (passive Nuclei technology fingerprint)",
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

    status_match = re.match(r"\s*(\d+)", response.status)
    if not status_match or int(status_match.group(1)) != 200:
        return []
    lowered_body = body.lower()
    if (
        "<title>jitsi meet</title>" not in lowered_body
        and "jitsimeetjs.app" not in lowered_body
    ):
        return []

    extracted = re.findall(r"lib\-jitsi\-meet\.min\.js\?v=([0-9]+)", body)
    return [finding("jitsi-meet-detect", extracted=list(dict.fromkeys(extracted)))]
