metadata = {
    "id": "plesk-eol",
    "about": "Plesk End-of-Life - Detect (passive Nuclei technology fingerprint)",
    "authors": [
        "ProjectDiscovery contributors",
        "Cruster",
    ],
    "references": [
        "https://github.com/projectdiscovery/nuclei-templates/tree/main/http/technologies",
    ],
    "tags": [
        "technology",
        "discovery",
        "nuclei",
        "passive",
    ],
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
    if target not in {"/", "/login_up.php"}:
        return []
    body = base64.b64decode(response.body).decode("latin-1")
    versions = []
    patterns = (
        r"(?i)<title>Plesk Obsidian ([0-9.]+)",
        r"(?i)Plesk Obsidian ([0-9.]+)",
    )
    for pattern in patterns:
        versions.extend(match.group(1) for match in re.finditer(pattern, body))
    if versions and tuple(map(int, versions[0].split("."))) < (18, 0, 75):
        return [finding("plesk-eol", extracted=list(dict.fromkeys(versions)))]
    return []
