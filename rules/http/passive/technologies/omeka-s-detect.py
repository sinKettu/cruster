metadata = {
    "id": "omeka-s-detect",
    "about": "Omeka S - Detect (passive Nuclei technology fingerprint)",
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
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    status = re.match(r"\s*(\d+)", response.status)
    has_asset = (
        "/application/asset/" in body or "&#x2F;application&#x2F;asset&#x2F;" in body
    )
    if (
        status
        and int(status.group(1)) == 200
        and has_asset
        and re.search(r"(?i)<title>[^<]*Omeka S[^<]*</title>", body)
    ):
        return [finding("omeka-s-detect", extracted=[])]
    return []
