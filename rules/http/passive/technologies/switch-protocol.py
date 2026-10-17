metadata = {
    "id": "switch-protocol",
    "about": "Switching Protocol Detection (passive Nuclei technology fingerprint)",
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
    try:
        headers = "\n".join(
            item["name"] + ": " + base64.b64decode(item["value"]).decode("latin-1")
            for item in response.headers
        )
    except Exception:
        headers = ""
    status = re.match(r"\s*(\d+)", response.status)
    lowered = headers.lower()
    matched = any(
        (
            status is not None and int(status.group(1)) == 101,
            "switching protocols" in lowered,
            "connection: upgrade" in lowered,
            "upgrade:" in lowered,
        )
    )
    if not matched:
        return []
    upgrades = []
    for item in response.headers:
        if item["name"].lower().replace("-", "_") == "upgrade":
            try:
                upgrades.append(base64.b64decode(item["value"]).decode("latin-1"))
            except Exception:
                pass
    return [finding("switch-protocol", extracted=upgrades)]
