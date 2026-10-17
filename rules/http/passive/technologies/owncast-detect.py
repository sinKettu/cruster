metadata = {
    "id": "owncast-detect",
    "about": "Owncast - Detect (passive Nuclei technology fingerprint)",
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
    markers = (
        "<title>Owncast</title>",
        'href="/api/auth/provider/indieauth"',
        "/js/web_modules/videojs/video-js.min.css",
    )
    if (
        status
        and int(status.group(1)) == 200
        and all(marker in body for marker in markers)
    ):
        return [finding("owncast-detect", extracted=[])]
    return []
