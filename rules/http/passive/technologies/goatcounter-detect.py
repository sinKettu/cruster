metadata = {
    "id": "goatcounter-detect",
    "about": "GoatCounter - Detect (passive Nuclei technology fingerprint)",
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
    markers = (
        "<title>GoatCounter</title>",
        'href="https://github.com/arp242/goatcounter"',
        'action="/user/requestlogin"',
    )
    return [finding("goatcounter-detect")] if all(x in body for x in markers) else []
