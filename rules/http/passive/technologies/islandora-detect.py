metadata = {
    "id": "islandora-detect",
    "about": "Islandora - Detect (passive Nuclei technology fingerprint)",
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
    if not status or int(status.group(1)) != 200:
        return []

    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""

    islandora_8 = (
        "islandora: http://islandora.ca/" in body
        and "fedora: http://fedora.info/definitions/v4/repository#" in body
    )
    islandora_7 = "/islandora-" in body and "islandora_solr" in body
    if not (islandora_8 or islandora_7):
        return []
    if 'content="Drupal ' not in body:
        return []

    return [finding("islandora-detect")]
