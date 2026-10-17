metadata = {
    "id": "xampp-default-page",
    "about": "XAMPP Default Page (passive Nuclei technology fingerprint)",
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
    path = request.uri.split("://", 1)[-1]
    if "/" in path:
        path = path[path.find("/") :]
    else:
        path = "/"
    path = path.split("#", 1)[0].split("?", 1)[0] or "/"
    if path != "/dashboard/" or not response.status.startswith("200"):
        return []

    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []

    if "<title>Welcome to XAMPP</title>" not in body:
        return []

    return [finding("xampp-default-page")]
