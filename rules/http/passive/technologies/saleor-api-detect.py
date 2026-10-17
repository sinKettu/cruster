metadata = {
    "id": "saleor-api-detect",
    "about": "Saleor API - Detect (passive Nuclei technology fingerprint)",
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
    except Exception:
        body = ""

    status_match = re.match(r"\s*(\d+)", response.status)
    status_code = int(status_match.group(1)) if status_match else 0
    required_words = (
        "<title>Saleor e-commerce</title>",
        "Your Saleor instance is ready to work",
        "GraphQL Playground",
    )
    if status_code != 200 or not all(word in body for word in required_words):
        return []

    return [finding("saleor-api-detect", extracted=[])]
