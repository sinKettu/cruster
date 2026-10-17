metadata = {
    "id": "dell-dpi-panel",
    "about": "Dell DPI Remote Power Management - Detect (passive Nuclei technology fingerprint)",
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
    body = base64.b64decode(response.body).decode("latin-1").lower()
    words = (
        'title="dell dpi remote power management',
        "<title>dell dpi remote power management",
    )
    if (
        target == "/index2.html"
        and status
        and int(status.group(1)) == 200
        and any(word in body for word in words)
    ):
        return [finding("dell-dpi-panel")]
    return []
