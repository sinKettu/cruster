metadata = {
    "id": "gladys-assistant-detect",
    "about": "Gladys Assistant - Detect (passive Nuclei technology fingerprint)",
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
    if not status or int(status.group(1)) != 200:
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []
    markers = (
        "<title>Gladys Assistant</title>",
        "/assets/splash/apple-splash-2048-2732.jpg",
    )
    return (
        [finding("gladys-assistant-detect")] if all(x in body for x in markers) else []
    )
