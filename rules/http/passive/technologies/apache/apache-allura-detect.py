metadata = {
    "id": "apache-allura-detect",
    "about": "Apache Allura - Detection (passive Nuclei technology fingerprint)",
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
    path = request.uri.split("://", 1)[-1]
    path = path[path.find("/") :] if "/" in path else "/"
    path = path.split("#", 1)[0].split("?", 1)[0] or "/"
    if path != "/neighborhood" or not response.status.startswith("200"):
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    if "<title>All Neighborhoods</title>" in body and ">Apache Allura</a>" in body:
        return [finding("apache-allura-detect")]
    return []
