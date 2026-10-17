import base64
import re
from cruster import HttpRequest, HttpResponse, finding

metadata = {
    "id": "publii-detect",
    "about": "Publii - Detect (passive Nuclei technology fingerprint)",
    "authors": ["ProjectDiscovery contributors", "Cruster"],
    "references": [
        "https://github.com/projectdiscovery/nuclei-templates/tree/main/http/technologies"
    ],
    "tags": ["technology", "discovery", "nuclei", "passive"],
    "severity": "info",
    "type": "passive",
}


def scan(request: HttpRequest, response: HttpResponse):
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    status = re.match(r"\s*(\d+)", response.status)
    style = re.search(r"""href=["'][^"']*/assets/css/style\.css\?v=[a-f0-9]+""", body)
    detected = (
        status is not None
        and int(status.group(1)) == 200
        and '<meta name="generator" content="Publii Open-Source CMS for Static Site">'
        in body
        and style is not None
    )
    return [finding("publii-detect", extracted=[])] if detected else []
