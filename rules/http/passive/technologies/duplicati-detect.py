metadata = {
    "id": "duplicati-detect",
    "about": "Duplicati - Detect (passive Nuclei technology fingerprint)",
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
    del request
    status = re.match(r"\s*(\d+)", response.status)
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    markers = ("<title>Duplicati Login</title>", "login/login.js", "<h2>Duplicati</h2>")
    detected = (
        status
        and int(status.group(1)) == 200
        and all(marker in body for marker in markers)
    )
    return [finding("duplicati-detect")] if detected else []
