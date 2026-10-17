metadata = {
    "id": "silverstripe-detect",
    "about": "SilverStripe Detection (passive Nuclei technology fingerprint)",
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
    if status is None or int(status.group(1)) not in (200, 403):
        return []
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
    combined = "\n".join(headers) + "\n\n" + body
    signatures = (
        "powered by silverstripe",
        "x-powered-by-silverstripe",
    )
    if not any(value in combined.lower() for value in signatures):
        return []
    patterns = (
        r"SilverStripe ([0-9.]+)",
        r"Powered by SilverStripe ([0-9.]+)",
        r"silverstripe-site-id=([a-z0-9]+)",
        r"silverstripe\.org/([^/]+)",
    )
    extracted = []
    for pattern in patterns:
        extracted.extend(match.group(0) for match in re.finditer(pattern, body))
    return [finding("silverstripe-detect", extracted=list(dict.fromkeys(extracted)))]
