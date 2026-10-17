metadata = {
    "id": "leantime-detect",
    "about": "Leantime - Detect (passive Nuclei technology fingerprint)",
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
    if target != "/auth/login":
        return []
    status = re.match(r"\s*(\d+)", response.status)
    if not status or int(status.group(1)) != 200:
        return []
    body = base64.b64decode(response.body).decode("latin-1")
    markers = (
        'content="Leantime',
        'name="leantime-version',
        "window.leantime.currentProject",
        ".leantimeLogo {",
        'class="leantimeLogo',
    )
    if not any(marker in body for marker in markers):
        return []
    versions = [
        match.group(1)
        for match in re.finditer(
            r'<meta name="leantime-version" content="(\d+\.\d+\.\d+)">', body
        )
    ]
    return [finding("leantime-detect", extracted=versions)]
