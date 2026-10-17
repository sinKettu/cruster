metadata = {
    "id": "lotus-domino-version",
    "about": "Lotus Domino Version Extractor (passive Nuclei technology fingerprint)",
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
    paths = {
        "/homepage.nsf",
        "/iNotes/Forms5.nsf",
        "/iNotes/Forms6.nsf",
        "/iNotes/Forms7.nsf",
        "/iNotes/Forms8.nsf",
        "/iNotes/Forms85.nsf",
        "/iNotes/Forms9.nsf",
    }
    target = request.uri.split("://", 1)[-1]
    target = target[target.find("/") :] if "/" in target else "/"
    target = target.split("#", 1)[0].split("?", 1)[0] or "/"
    if target not in paths:
        return []
    status = re.match(r"\s*(\d+)", response.status)
    if not status or int(status.group(1)) != 200:
        return []
    body = base64.b64decode(response.body).decode("latin-1")
    pattern = (
        r"((Domino\s[A-Za-z]+\s[0-9\.]{1,3})|(Forms[0-9\.]{1,3}\.nsf\?OpenDatabase))"
    )
    matches = list(re.finditer(pattern, body))
    if not matches:
        return []
    return [
        finding("lotus-domino-version", extracted=[match.group(0) for match in matches])
    ]
