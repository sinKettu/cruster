metadata = {
    "id": "kubernetes-version",
    "about": "Kubernetes Version Exposure (passive Nuclei technology fingerprint)",
    "authors": ["ProjectDiscovery contributors", "Cruster"],
    "references": [
        "https://github.com/projectdiscovery/nuclei-templates/tree/main/http/technologies"
    ],
    "tags": ["technology", "discovery", "nuclei", "passive"],
    "severity": "info",
    "type": "passive"
}

import base64

from cruster import HttpRequest, HttpResponse, finding


def scan(request: HttpRequest, response: HttpResponse):
    target = request.uri.split("://", 1)[-1]
    path = target[target.find("/") :] if "/" in target else "/"
    path = path.split("#", 1)[0].split("?", 1)[0] or "/"
    if path != "/version":
        return []

    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []

    markers = ["gitVersion", "goVersion", "platform"]
    if not all(marker in body for marker in markers):
        return []

    return [finding("kubernetes-version", extracted=[])]
