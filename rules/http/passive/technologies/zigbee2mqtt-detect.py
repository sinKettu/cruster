metadata = {
    "id": "zigbee2mqtt-detect",
    "about": "Zigbee2MQTT - Detect (passive Nuclei technology fingerprint)",
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
        return []

    if "<title>Zigbee2MQTT</title>" not in body:
        return []

    favicon_pattern = r"href=\"\./assets/favicon-[a-f0-9]+\.ico\""
    if not re.search(favicon_pattern, body):
        return []

    return [finding("zigbee2mqtt-detect")]
