metadata = {
    "id": "funkwhale-detect",
    "about": "Funkwhale - Detect (passive Nuclei technology fingerprint)",
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
    if not status or int(status.group(1)) != 200:
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    if "<title>Funkwhale</title>" not in body:
        return []
    pattern = r'<meta\s+name=["\']?generator["\']?\s+content=["\']?Funkwhale["\']?'
    return [finding("funkwhale-detect")] if re.search(pattern, body) else []
