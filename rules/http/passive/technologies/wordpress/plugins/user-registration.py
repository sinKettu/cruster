metadata = {
    "id": "user-registration",
    "about": "WordPress User Registration & Membership Plugin Detection (passive Nuclei technology fingerprint)",
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
    if target != "/wp-content/plugins/user-registration/readme.txt":
        return []
    status = re.match(r"\s*(\d+)", response.status)
    if status is None or int(status.group(1)) != 200:
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []
    match = re.search(r"(?i)Stable tag:\s*([\d.]+)", body)
    if "User Registration" not in body or match is None:
        return []
    version = match.group(1)
    host = request.uri.split("://", 1)[-1].split("/", 1)[0]
    evidence = [
        version,
        "[User Registration] detected on " + host + " (version: " + version + ")",
    ]
    return [finding("user-registration", extracted=evidence)]
