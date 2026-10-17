metadata = {
    "id": "kanidm-detect",
    "about": "Kanidm - Detect (passive Nuclei technology fingerprint)",
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
    header_lines = []
    for header in response.headers:
        try:
            value = base64.b64decode(header["value"]).decode("latin-1")
        except Exception:
            continue
        header_lines.append(header["name"] + ": " + value)

    headers = "\n".join(header_lines).lower()
    status_match = re.match(r"\s*(\d+)", response.status)
    status_code = int(status_match.group(1)) if status_match else 0
    has_version = "x-kanidm-version:" in headers
    has_operation_id = "x-kanidm-opid:" in headers
    if not (has_version and has_operation_id and status_code == 303):
        return []

    return [finding("kanidm-detect")]
