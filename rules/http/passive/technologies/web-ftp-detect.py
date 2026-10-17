metadata = {
    "id": "web-ftp-detect",
    "about": "Web FTP Detection (passive Nuclei technology fingerprint)",
    "authors": ["ProjectDiscovery contributors", "Cruster"],
    "references": [
        "https://github.com/projectdiscovery/nuclei-templates/tree/main/http/technologies"
    ],
    "tags": ["technology", "discovery", "nuclei", "passive"],
    "severity": "info",
    "type": "passive",
}

import base64
from cruster import HttpRequest, HttpResponse, finding


def scan(request: HttpRequest, response: HttpResponse):
    if not request.uri.split("?", 1)[0].endswith("/cgi-bin/upload/web-ftp.cgi"):
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []
    if not response.status.startswith("200"):
        return []
    if "Web-FTP" not in body or "square login" not in body:
        return []
    return [finding("web-ftp-detect")]
