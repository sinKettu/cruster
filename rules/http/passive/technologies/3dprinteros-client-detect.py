metadata = {
    "id": "3dprinteros-client-detect",
    "about": "3DPrinterOS Client - Detect (passive Nuclei technology fingerprint)",
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
        body = base64.b64decode(response.body).decode("latin-1")
        headers = "\n".join(
            item["name"] + ": " + base64.b64decode(item["value"]).decode("latin-1")
            for item in response.headers
        )
    except Exception:
        return []
    status = re.match(r"\s*(\d+)", response.status)
    if (
        status
        and int(status.group(1)) == 401
        and "<title>3DPrinterOS Client</title>" in body
        and "aiohttp/" in headers
    ):
        return [finding("3dprinteros-client-detect")]
    return []
