metadata = {
    "id": "coder-detect",
    "about": "Coder - Detect (passive Nuclei technology fingerprint)",
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
    status = re.match(r"\s*(\d+)", response.status)
    if not status or int(status.group(1)) != 200:
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    if "<title>Coder</title>" not in body or 'meta property="build-info"' not in body:
        return []
    for item in response.headers:
        try:
            header = (
                item["name"] + ": " + base64.b64decode(item["value"]).decode("latin-1")
            )
        except Exception:
            continue
        if "X-Coder-Build-Version:" in header:
            return [finding("coder-detect")]
    return []
