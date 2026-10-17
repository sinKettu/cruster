metadata = {
    "id": "topdesk-detect",
    "about": "TOPdesk - Detect (passive Nuclei technology fingerprint)",
    "authors": ["ProjectDiscovery contributors", "Cruster"],
    "references": [
        "https://github.com/projectdiscovery/nuclei-templates/tree/main/http/technologies"
    ],
    "tags": ["technology", "discovery", "nuclei", "passive"],
    "severity": "info",
    "type": "passive"
}

import base64
from datetime import datetime, timezone
import re

from cruster import HttpRequest, HttpResponse, finding


def scan(_request: HttpRequest, response: HttpResponse):
    if response.status.lstrip().split(" ", 1)[0] != "200":
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []
    if 'href="/tas/' not in body and "topdeskidploginforwardurl" not in body:
        return []

    extracted = []
    timestamp_match = re.search(r"auth\.css\?timestamp=(\d{10})", body)
    if timestamp_match:
        try:
            release_date = datetime.fromtimestamp(
                int(timestamp_match.group(1)), timezone.utc
            ).strftime("%Y-%M-%D")
            extracted.append("Release date: " + release_date)
        except (OverflowError, OSError, ValueError):
            pass
    return [finding("topdesk-detect", extracted=extracted)]
