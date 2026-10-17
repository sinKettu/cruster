metadata = {
    "id": "ivanti-xtraction-detect",
    "about": "Ivanti Xtraction - Detect (passive Nuclei technology fingerprint)",
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
    status = re.match(r"\s*(\d+)", response.status)
    body = base64.b64decode(response.body).decode("latin-1")
    words = (
        "<title>Xtraction</title>",
        'application-name" content="Xtraction',
        "dev-extreme-ivanti.css",
    )
    if (
        target == "/xtraction"
        and status
        and int(status.group(1)) == 200
        and all(word in body for word in words)
    ):
        return [finding("ivanti-xtraction-detect")]
    return []
