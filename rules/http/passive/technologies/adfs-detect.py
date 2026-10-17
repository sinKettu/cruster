metadata = {
    "id": "adfs-detect",
    "about": "ADFS Detect (passive Nuclei technology fingerprint)",
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
    target = request.uri.split("://", 1)[-1]
    target = (
        (target[target.find("/") :] if "/" in target else "/")
        .split("#", 1)[0]
        .split("?", 1)[0]
    )
    if target != "/adfs/ls/idpinitiatedsignon.aspx":
        return []

    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []

    if "/adfs/portal/css/style.css" not in body:
        return []

    domains = [
        match.group(1)
        for match in re.finditer(r"var\s+userNameValue\s*=\s*'([^\\]*)\\", body)
    ]
    return [finding("adfs-detect", extracted=domains)]
