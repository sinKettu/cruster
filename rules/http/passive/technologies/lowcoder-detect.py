metadata = {
    "id": "lowcoder-detect",
    "about": "Lowcoder - Detect (passive Nuclei technology fingerprint)",
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
    target = target.split("#", 1)[0].split("?", 1)[0] or "/"
    status = re.match(r"\s*(\d+)", response.status)
    if not status or int(status.group(1)) != 200:
        return []
    body = base64.b64decode(response.body).decode("latin-1")
    title = (
        '<title data-react-helmet="true">Lowcoder | rapid App & VideoMeeting builder for '
        "everyone.</title>"
    )
    if title in body:
        return [finding("lowcoder-detect")]
    if target != "/manifest.json" or "../lowcoder-design/src/" not in body:
        return []
    headers = "\n".join(
        header["name"] + ": " + base64.b64decode(header["value"]).decode("latin-1")
        for header in response.headers
    )
    if "Content-Type: application/json" in headers:
        return [finding("lowcoder-detect")]
    return []
