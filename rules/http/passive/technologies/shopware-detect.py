metadata = {
    "id": "shopware-detect",
    "about": "Shopware CMS detect (passive Nuclei technology fingerprint)",
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
    if target not in ("/admin", "/backend"):
        return []
    status = re.match(r"\s*(\d+)", response.status)
    if status is None or int(status.group(1)) != 200:
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    signatures = (
        "Realisiert mit Shopware",
        "Realised with Shopware",
        "Shopware Administration (c) shopware AG",
        "<title>Shopware 5 - Backend (c) shopware AG</title>",
        "Shopware.Application.start",
    )
    if not any(signature in body for signature in signatures):
        return []
    return [finding("shopware-detect", extracted=[])]
