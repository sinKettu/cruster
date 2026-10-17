metadata = {
    "id": "default-parallels-plesk",
    "about": "Default Parallels Plesk Panel Page (passive Nuclei technology fingerprint)",
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
    del request
    status = re.match(r"\s*(\d+)", response.status)
    try:
        body = base64.b64decode(response.body).decode("latin-1").lower()
    except Exception:
        body = ""
    markers = (
        "<title>default parallels plesk panel page</title>",
        "welcome to parallels",
    )
    detected = (
        status
        and int(status.group(1)) == 200
        and any(marker in body for marker in markers)
    )
    return [finding("default-parallels-plesk")] if detected else []
