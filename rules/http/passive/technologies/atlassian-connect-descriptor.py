metadata = {
    "id": "atlassian-connect-descriptor",
    "about": "Atlassian Connect Descriptor - Detect (passive Nuclei technology fingerprint)",
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
    target = target.split("?", 1)[0].split("#", 1)[0]
    status = re.match(r"\s*(\d+)", response.status)
    if (
        target != "/atlassian-connect.json"
        or status is None
        or int(status.group(1)) != 200
    ):
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1")
        headers = "\n".join(
            header["name"] + ": " + base64.b64decode(header["value"]).decode("latin-1")
            for header in response.headers
        )
    except Exception:
        return []

    markers = ('"name":', '"vendor":', '"key":', '"baseUrl":', '"authentication":')
    if not all(marker in body for marker in markers):
        return []
    if "application/json" not in headers:
        return []

    return [finding("atlassian-connect-descriptor")]
