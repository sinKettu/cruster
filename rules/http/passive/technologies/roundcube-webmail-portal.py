metadata = {
    "id": "roundcube-webmail-portal",
    "about": "Roundcube webmail (passive Nuclei technology fingerprint)",
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
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""

    status_match = re.match(r"\s*(\d+)", response.status)
    if status_match is None or int(status_match.group(1)) != 200:
        return []

    raw_version_match = re.search(r'"rcversion":(\d+)', body)
    if raw_version_match is None:
        return []

    if re.search(r"(?i)roundcube", body) is None:
        return []

    raw_version = raw_version_match.group(1)
    major = int(raw_version[0:1])
    minor = int(raw_version[1:3])
    patch = int(raw_version[3:])
    version = f"{major}.{minor}.{patch}"

    return [finding("roundcube-webmail-portal", extracted=[version])]
