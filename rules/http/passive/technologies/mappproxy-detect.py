metadata = {
    "id": "mappproxy-detect",
    "about": "Mapproxy - Detect (passive Nuclei technology fingerprint)",
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

    body = base64.b64decode(response.body).decode("latin-1")
    headers = "\n".join(
        item["name"] + ": " + base64.b64decode(item["value"]).decode("latin-1")
        for item in response.headers
    )
    if "text/html" not in headers or not any(
        value in body for value in ("Welcome to MapProxy", "MapProxy Demo")
    ):
        return []
    # The source extractor has a non-leading inline flag and therefore yields
    # no values under the importer's exception-tolerant regex semantics.
    return [finding("mappproxy-detect")]
