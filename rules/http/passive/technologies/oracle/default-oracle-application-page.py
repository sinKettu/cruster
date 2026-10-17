metadata = {
    "id": "default-oracle-application-page",
    "about": "Oracle Application Server Containers (passive Nuclei technology fingerprint)",
    "authors": ["ProjectDiscovery contributors", "Cruster"],
    "references": [
        "https://github.com/projectdiscovery/nuclei-templates/tree/main/http/technologies"
    ],
    "tags": ["technology", "discovery", "nuclei", "passive"],
    "severity": "info",
    "type": "passive"
}
import base64, re
from cruster import HttpRequest, HttpResponse, finding


def scan(request: HttpRequest, response: HttpResponse):
    try:
        b = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        b = ""
    if "<TITLE>Oracle Application Server" not in b:
        return []
    values = [m.group(1) for m in re.finditer(r"(?i)\s+\(([0-9.]+)\)", b)]
    return [finding("default-oracle-application-page", extracted=values)]
