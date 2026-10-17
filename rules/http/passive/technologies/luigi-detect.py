metadata = {
    "id": "luigi-detect",
    "about": "Luigi - Detect (passive Nuclei technology fingerprint)",
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
    body = base64.b64decode(response.body).decode("latin-1")
    assets = ("css/luigi.css", "js/luigi.js", "js/visualiserApp.js")
    if "<title>Luigi Task Visualiser</title>" in body and all(
        asset in body for asset in assets
    ):
        return [finding("luigi-detect")]
    return []
