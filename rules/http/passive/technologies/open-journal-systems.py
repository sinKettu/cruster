metadata = {
    "id": "open-journal-systems",
    "about": "Open Journal Systems Detect (passive Nuclei technology fingerprint)",
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
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    status = re.match(r"\s*(\d+)", response.status)
    if (
        not status
        or int(status.group(1)) != 200
        or not re.search(
            r'(?i).*<meta.name="generator".content="Open.Journal.Systems.*>', body
        )
    ):
        return []
    versions = re.findall(
        r'(?i).*<meta.name="generator".content="Open.Journal.Systems.([\d.|\d]+)', body
    )
    return [finding("open-journal-systems", extracted=versions)]
