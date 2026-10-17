metadata = {
    "id": "wix-detect",
    "about": "Wix Detection (passive Nuclei technology fingerprint)",
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
    markers = ["powered by wix", "x-wix-request-id", "wix static site", "wixsite.com"]
    allowed_status = any(
        response.status.startswith(code) for code in ["200", "403", "404"]
    )
    if not allowed_status or not any(
        marker in response_text.lower() for marker in markers
    ):
        return []
    patterns = [
        r"Wix.com ([0-9.]+)",
        r"Wix version ([0-9.]+)",
        r"Wix Static Site Version ([0-9.]+)",
        r"wix-id=([a-z0-9]+)",
        r'siteid="([a-zA-Z0-9]+)"',
    ]
    extracted = []
    for pattern in patterns:
        extracted.extend(match.group(1) for match in re.finditer(pattern, body))
    return [finding("wix-detect", extracted=extracted)]
