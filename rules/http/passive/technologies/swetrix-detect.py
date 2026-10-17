metadata = {
    "id": "swetrix-detect",
    "about": "Swetrix - Detect (passive Nuclei technology fingerprint)",
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
        headers = "\n".join(
            item["name"] + ": " + base64.b64decode(item["value"]).decode("latin-1")
            for item in response.headers
        )
    except Exception:
        body = ""
        headers = ""
    status = re.match(r"\s*(\d+)", response.status)
    body_match = all(
        value in body
        for value in (
            "Swetrix Community Edition</title>",
            '<meta name="twitter:site" content="@swetrix"',
            'href="/favicon-ce.ico"',
        )
    )
    matched = (
        status is not None
        and int(status.group(1)) == 200
        and body_match
        and "x-powered-by: mountain dew" in headers.lower()
    )
    return [finding("swetrix-detect", extracted=[])] if matched else []
