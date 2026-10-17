metadata = {
    "id": "domibus-detect",
    "about": "Domibus - Detect (passive Nuclei technology fingerprint)",
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
    if target not in ("/domibus/rest/application/info", "/domibus/"):
        return []
    status = re.match(r"\s*(\d+)", response.status)
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    if (
        not status
        or int(status.group(1)) != 200
        or not any(x in body.lower() for x in ("<title>domibus", "domibus-msh"))
    ):
        return []
    version = re.search(r'"versionNumber":\s*"([0-9.]+)"', body)
    return [finding("domibus-detect", extracted=[version.group(1)] if version else [])]
