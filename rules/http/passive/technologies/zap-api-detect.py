metadata = {
    "id": "zap-rest-api-detect",
    "about": "ZAP Rest API Server Running (passive Nuclei technology fingerprint)",
    "authors": ["ProjectDiscovery contributors", "Cruster"],
    "references": [
        "https://github.com/projectdiscovery/nuclei-templates/tree/main/http/technologies"
    ],
    "tags": ["technology", "discovery", "nuclei", "passive"],
    "severity": "info",
    "type": "passive",
}

import base64

from cruster import HttpRequest, HttpResponse, finding


def scan(request: HttpRequest, response: HttpResponse):
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""

    headers = []
    for item in response.headers:
        try:
            value = base64.b64decode(item["value"]).decode("latin-1")
        except Exception:
            continue
        headers.append(item["name"] + ": " + value)

    response_text = "\n".join(headers) + "\n\n" + body
    markers = (
        "<title>ZAP API UI</title>",
        "Welcome to the OWASP Zed Attack Proxy (ZAP)",
        "Access-Control-Allow-Headers: ZAP-Header",
    )
    if not any(marker in response_text for marker in markers):
        return []

    return [finding("zap-rest-api-detect")]
