metadata = {
    "id": "sylius-detect",
    "about": "Sylius - Detect (passive Nuclei technology fingerprint)",
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
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    status = re.match(r"\s*(\d+)", response.status)
    structure_match = re.search(
        r'(?:id="sylius-cart-button"|class="[^"]*sylius-product-name|data-route="sylius_shop_|/build/sylius-shop-|<title>Sylius \|)',
        body,
    )
    matched = (
        status is not None
        and int(status.group(1)) in (200, 404)
        and "sylius" in body.lower()
        and structure_match
    )
    return [finding("sylius-detect", extracted=[])] if matched else []
