metadata = {
    "id": "pbootcms-detect",
    "about": "PbootCMS Detect (passive Nuclei technology fingerprint)",
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
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    releases = [match.group(1) for match in re.finditer(r'title="(.*)正式发布">', body)]
    detected = (
        'content="PbootCMS' in body
        or re.search(r"<title>PbootCMS(.*)</title>", body) is not None
    )
    return [finding("pbootcms-detect", extracted=releases)] if detected else []
