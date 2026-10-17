metadata = {
    "id": "tileserver-gl",
    "about": "TileServer GL Page - Detect (passive Nuclei technology fingerprint)",
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


def scan(_request: HttpRequest, response: HttpResponse):
    if response.status.lstrip().split(" ", 1)[0] != "200":
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []
    title = "<title>TileServer GL - Server for vector and raster maps with GL styles</title>"
    if title not in body:
        return []
    versions = [
        match.group(1)
        for match in re.finditer(r"Powered by TileServer GL \(([a-z- 0-9.]+)\)", body)
    ]
    return [finding("tileserver-gl", extracted=versions)]
