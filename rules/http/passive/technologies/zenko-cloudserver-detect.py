metadata = {
    "id": "zenko-cloudserver-detect",
    "about": "Zenko CloudServer - Detect (passive Nuclei technology fingerprint)",
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
    if not status or int(status.group(1)) != 400:
        return []

    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []

    headers = []
    for item in response.headers:
        try:
            value = base64.b64decode(item["value"]).decode("latin-1")
        except Exception:
            continue
        headers.append(item["name"] + ": " + value)

    header = "\n".join(headers)
    header_markers = ("Server: ScalityS3", "X-Scal-Request-Id:")
    if not all(marker in header for marker in header_markers):
        return []

    body_markers = (
        '"code":"InvalidParameterValue"',
        '"message":"An invalid or out-of-range value was supplied for the input parameter."',
    )
    if not all(marker in body for marker in body_markers):
        return []

    return [finding("zenko-cloudserver-detect")]
