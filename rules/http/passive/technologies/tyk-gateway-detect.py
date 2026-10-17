metadata = {
    "id": "tyk-gateway-detect",
    "about": "Tyk API Gateway - Detection (passive Nuclei technology fingerprint)",
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
    target = request.uri.split("://", 1)[-1]
    target = target[target.find("/") :] if "/" in target else "/"
    target = target.split("#", 1)[0].split("?", 1)[0] or "/"
    if target != "/hello":
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1")
        content_types = [
            base64.b64decode(item["value"]).decode("latin-1")
            for item in response.headers
            if item["name"].lower() == "content-type"
        ]
    except Exception:
        return []
    if "Tyk GW" not in body or "description" not in body:
        return []
    if "application/json" not in "\n".join(content_types):
        return []
    versions = re.findall(r'"version":"[^"]*"', body)
    return [finding("tyk-gateway-detect", extracted=versions)]
