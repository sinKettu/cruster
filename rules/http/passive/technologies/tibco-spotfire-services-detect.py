metadata = {
    "id": "tibco-spotfire-services-detect",
    "about": "TIBCO Spotfire Statistics Services - Detect (passive Nuclei technology fingerprint)",
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


def scan(_request: HttpRequest, response: HttpResponse):
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    headers = []
    for header in response.headers:
        try:
            value = base64.b64decode(header["value"]).decode("latin-1")
        except Exception:
            value = ""
        headers.append(header["name"] + ": " + value)

    body_indicators = (
        "tibco spotfire statistics services",
        "spotfire statistics services",
    )
    if not (
        any(indicator in body.lower() for indicator in body_indicators)
        or "TIBCO Spotfire Server" in "\n".join(headers)
    ):
        return []
    versions = [
        match.group(1) for match in re.finditer(r"Version\s+([0-9.\s\(\)V]+)", body)
    ]
    return [finding("tibco-spotfire-services-detect", extracted=versions)]
