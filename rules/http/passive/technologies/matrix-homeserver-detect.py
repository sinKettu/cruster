metadata = {
    "id": "matrix-homeserver-detect",
    "about": "Matrix Homeserver - Version Detection (passive Nuclei technology fingerprint)",
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
    target = target[target.find("/") :] if "/" in target else "/"
    target = target.split("#", 1)[0].split("?", 1)[0] or "/"
    if target != "/_matrix/federation/v1/version":
        return []

    body = base64.b64decode(response.body).decode("latin-1")
    content_types = [
        base64.b64decode(item["value"]).decode("latin-1")
        for item in response.headers
        if item["name"].lower() == "content-type"
    ]
    if all(value in body for value in ('"server":', '"name":', '"version":')) and any(
        "application/json" in value for value in content_types
    ):
        return [finding("matrix-homeserver-detect")]
    return []
