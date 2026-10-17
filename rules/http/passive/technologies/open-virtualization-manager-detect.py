metadata = {
    "id": "open-virtualization-manager-detect",
    "about": "Open Virtualization Manager Detection (passive Nuclei technology fingerprint)",
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
    path = request.uri.split("://", 1)[-1]
    path = path[path.find("/") :] if "/" in path else "/"
    path = path.split("#", 1)[0].split("?", 1)[0] or "/"
    if path != "/ovirt-engine/":
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    markers = (
        "Welcome to Open Virtualization Manager",
        "<title>Ovirt-Engine</title>",
        "<title>oVirt Engine</title>",
        "<title>oVirt Engine Error</title>",
    )
    if not any(marker in body for marker in markers):
        return []
    versions = re.findall(r'<span class="version-text">(.+)<\/span>', body)
    return [finding("open-virtualization-manager-detect", extracted=versions)]
