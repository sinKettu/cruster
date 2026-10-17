metadata = {
    "id": "twenty-detect",
    "about": "Twenty - Detect (passive Nuclei technology fingerprint)",
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
    target = request.uri.split("://", 1)[-1]
    target = target[target.find("/") :] if "/" in target else "/"
    target = target.split("#", 1)[0].split("?", 1)[0] or "/"
    if target != "/welcome" or response.status.lstrip().split(" ", 1)[0] != "200":
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1").lower()
    except Exception:
        return []
    if "<title>twenty</title>" not in body or "open-source crm" not in body:
        return []
    return [finding("twenty-detect")]
