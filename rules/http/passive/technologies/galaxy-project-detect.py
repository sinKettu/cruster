metadata = {
    "id": "galaxy-project-detect",
    "about": "Galaxy Project - Detect (passive Nuclei technology fingerprint)",
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
        return []
    markers = ("/static/dist/base.css", "Javascript Required for Galaxy")
    title = re.search(
        r"<title>\s*Galaxy(?:\s*\|[^<]*)?\s*</title>", body, re.IGNORECASE | re.DOTALL
    )
    return (
        [finding("galaxy-project-detect")]
        if all(x in body for x in markers) and title
        else []
    )
