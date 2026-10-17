metadata = {
    "id": "matrix-detect",
    "about": "Matrix Server Detect (passive Nuclei technology fingerprint)",
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
    target = target[target.find("/") :] if "/" in target else "/"
    target = target.split("#", 1)[0].split("?", 1)[0] or "/"
    if target not in {"/.well-known/matrix/server", "/.well-known/matrix/client"}:
        return []

    body = base64.b64decode(response.body).decode("latin-1")
    headers = "\n".join(
        item["name"] + ": " + base64.b64decode(item["value"]).decode("latin-1")
        for item in response.headers
    )
    status = re.match(r"\s*(\d+)", response.status)
    if (
        status
        and int(status.group(1)) == 200
        and "text/html" not in headers
        and re.search(r'"m\.([a-z]+)":', body)
    ):
        return [finding("matrix-detect")]
    return []
