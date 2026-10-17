metadata = {
    "id": "flipt-detect",
    "about": "Flipt - Detect (passive Nuclei technology fingerprint)",
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
    del request
    status = re.match(r"\s*(\d+)", response.status)
    if status is None or int(status.group(1)) != 200:
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []
    if not all(
        marker in body
        for marker in (
            "<title>Flipt</title>",
            "Javascript is required for the Flipt UI",
        )
    ):
        return []
    headers = []
    for header in response.headers:
        try:
            value = base64.b64decode(header["value"]).decode("latin-1")
        except Exception:
            value = ""
        headers.append(header["name"] + ": " + value)
    text = "\n".join(headers).lower()
    if "content-security-policy:" not in text or "frame-ancestors 'none'" not in text:
        return []
    return [finding("flipt-detect")]
