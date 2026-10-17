metadata = {
    "id": "milvus-detect",
    "about": "Milvus - Detect (passive Nuclei technology fingerprint)",
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
    body = base64.b64decode(response.body).decode("latin-1")
    headers = "\n".join(
        item["name"] + ": " + base64.b64decode(item["value"]).decode("latin-1")
        for item in response.headers
    )
    titles = (
        "<title>Milvus",
        "<title>Milvus Demo",
        "<title>Milvus Insight",
        "<title>Milvus Enterprise Manager",
    )
    server = re.search(r"(?i)Server:\s*Milvus/([0-9]+(?:\.[0-9]+)*)", headers)
    if any(value in body for value in titles) or server:
        return [finding("milvus-detect", extracted=[server.group(1)] if server else [])]
    return []
