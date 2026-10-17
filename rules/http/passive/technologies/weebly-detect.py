metadata = {
    "id": "weebly-detect",
    "about": "Weebly Detection (passive Nuclei technology fingerprint)",
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
    for item in response.headers:
        try:
            value = base64.b64decode(item["value"]).decode("latin-1")
        except Exception:
            value = ""
        headers.append(item["name"] + ": " + value)
    response_text = "\n".join(headers) + "\n\n" + body
    markers = ["powered by weebly", "weebly.com", "x-powered-by-weebly"]
    if not (response.status.startswith("200") or response.status.startswith("403")):
        return []
    if not any(marker in response_text.lower() for marker in markers):
        return []
    patterns = [
        r"Weebly ([0-9.]+)",
        r"Powered by Weebly ([0-9.]+)",
        r"weebly-site-id=([a-z0-9]+)",
        r"weebly\.com/([^/]+)",
    ]
    extracted = []
    for pattern in patterns:
        extracted.extend(match.group(1) for match in re.finditer(pattern, body))
    return [finding("weebly-detect", extracted=extracted)]
