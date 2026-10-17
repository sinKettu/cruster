metadata = {
    "id": "gitea-detect",
    "about": "Gitea Detect (passive Nuclei technology fingerprint)",
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
    target = target.split("#", 1)[0].split("?", 1)[0]
    if target != "/user/login":
        return []
    status = re.match(r"\s*(\d+)", response.status)
    if status is None or int(status.group(1)) != 200:
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []
    if "Powered by Gitea" not in body and 'content="Gitea' not in body:
        return []
    extracted = []
    patterns = (
        r"Powered by Gitea(\s*)Version:(\s*)([\d.]+)",
        r"Gitea Version: ([0-9a-z+-.~]+)",
        r"theme-gitea-auto.css\?v=([0-9a-z+-.~]+)",
        r"encodeURIComponent.{2}([0-9a-z+-.~]+).{2}",
    )
    for pattern in patterns:
        for match in re.finditer(pattern, body):
            extracted.append(match.group(1))
    return [finding("gitea-detect", extracted=list(dict.fromkeys(extracted)))]
