metadata = {
    "id": "homegenie-detect",
    "about": "HomeGenie - Detect (passive Nuclei technology fingerprint)",
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
    status_match = re.match(r"\s*(\d+)", response.status)
    if status_match is None or int(status_match.group(1)) != 200:
        return []

    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []

    if re.search(r"(?is)<title>\s*HomeGenie(?:\s*-\s*YoT)?\s*</title>", body) is None:
        return []

    header_lines = []
    for header in response.headers:
        try:
            value = base64.b64decode(header["value"]).decode("latin-1")
        except Exception:
            return []
        header_lines.append(header["name"] + ": " + value)

    if "MIG WebService Gateway" not in "\n".join(header_lines):
        return []

    return [finding("homegenie-detect")]
