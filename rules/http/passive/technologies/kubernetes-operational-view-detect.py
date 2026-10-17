metadata = {
    "id": "kubernetes-operational-view-detect",
    "about": "Kubernetes Operational View Detect (passive Nuclei technology fingerprint)",
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

    status_match = re.match(r"\s*(\d+)", response.status)
    status_code = int(status_match.group(1)) if status_match else 0
    if status_code != 200:
        return []
    if "<title>Kubernetes Operational View" not in body:
        return []

    version_pattern = r"<title>Kubernetes Operational View (.+)</title>"
    versions = [match.group(1) for match in re.finditer(version_pattern, body)]
    return [finding("kubernetes-operational-view-detect", extracted=versions)]
