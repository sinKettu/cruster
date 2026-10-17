metadata = {
    "id": "evershop-detect",
    "about": "EverShop - Detect (passive Nuclei technology fingerprint)",
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
    del request
    status = re.match(r"\s*(\d+)", response.status)
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    words = "var eContext =" in body and "Evershop. All Rights Reserved." in body
    asset = re.search(r"<(?:script|link)[^>]+/assets/frontStore/", body)
    return (
        [finding("evershop-detect")]
        if status and int(status.group(1)) == 200 and words and asset
        else []
    )
