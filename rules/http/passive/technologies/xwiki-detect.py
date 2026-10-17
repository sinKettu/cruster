metadata = {
    "id": "xwiki-detect",
    "about": "XWiki - Detection (passive Nuclei technology fingerprint)",
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
    status = re.match(r"\s*(\d+)", response.status)
    if not status or int(status.group(1)) not in (200, 401):
        return []

    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []

    if "xwikiplatformversion" not in body:
        return []

    pages = [
        match.group(1)
        for match in re.finditer(r"<html.*?data-xwiki-page=\"(.*?)\"", body)
    ]
    return [finding("xwiki-detect", extracted=pages)]
