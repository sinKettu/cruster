metadata = {
    "id": "squarespace-detect",
    "about": "Squarespace Detection (passive Nuclei technology fingerprint)",
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
        body = ""
        headers = ""
    status = re.match(r"\s*(\d+)", response.status)
    response_text = headers + "\n\n" + body
    response_lower = response_text.lower()
    marker_match = any(
        marker.lower() in response_lower
        for marker in (
            "Powered by Squarespace",
            "X-Squarespace-Request-Id",
            "Squarespace Template",
        )
    )
    matched = status is not None and int(status.group(1)) in (200, 403) and marker_match
    if not matched:
        return []
    extracted = []
    patterns = (
        r"Squarespace ([0-9.]+)",
        r"Powered by Squarespace ([0-9.]+)",
        r"squarespace-site-id=([a-z0-9]+)",
        r'siteid="([a-zA-Z0-9]+)"',
        r"squarespace\.com/([^/]+)",
    )
    for pattern in patterns:
        extracted.extend(match.group(1) for match in re.finditer(pattern, body))
    return [finding("squarespace-detect", extracted=extracted)]
