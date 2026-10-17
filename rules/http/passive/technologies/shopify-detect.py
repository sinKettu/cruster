metadata = {
    "id": "shopify-detect",
    "about": "Shopify Detection (passive Nuclei technology fingerprint)",
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
        "x-shopify-stage",
        "powered by shopify",
        "cdn.shopify.com",
        "shopify store",
        "shopify liquid",
    )
    if not any(signature in combined.lower() for signature in signatures):
        return []
    patterns = (
        r"Shopify ([0-9.]+)",
        r"Shopify version ([0-9.]+)",
        r"Coded by Shopify ([0-9.]+)",
        r"Theme: ([A-Za-z0-9_-]+) Shopify",
        r"shop_id=([0-9]+)",
        r"shopify://([0-9a-zA-Z]+)",
    )
    extracted = []
    for pattern in patterns:
        extracted.extend(match.group(0) for match in re.finditer(pattern, body))
    return [finding("shopify-detect", extracted=list(dict.fromkeys(extracted)))]
