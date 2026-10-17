metadata = {
    "id": "tableau-server-detect",
    "about": "Salesforce Tableau Server - Detect (passive Nuclei technology fingerprint)",
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


def scan(_request: HttpRequest, response: HttpResponse):
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    headers = []
    for item in response.headers:
        try:
            value = base64.b64decode(item["value"]).decode("latin-1")
        except Exception:
            value = ""
        headers.append(item["name"] + ": " + value)
    header = "\n".join(headers)
    detected = any(
        marker in header for marker in ("X-Tableau: Tableau Server", "Server: Tableau")
    )
    detected = detected or any(
        marker in body for marker in ("VizPortalRun", "vizportal")
    )
    if not detected:
        return []
    builds = [
        match.group(1) for match in re.finditer(r'data-build[iI]d="([0-9a-z_]*)', body)
    ]
    return [finding("tableau-server-detect", extracted=builds)]
