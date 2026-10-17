metadata = {
    "id": "rasa-detect",
    "about": "Rasa Open Source - Detect (passive Nuclei technology fingerprint)",
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
    uri_without_scheme = request.uri.split("://", 1)[-1]
    if "/" in uri_without_scheme:
        request_target = uri_without_scheme[uri_without_scheme.find("/") :]
    else:
        request_target = "/"
    request_target = request_target.split("#", 1)[0]
    request_target = request_target.split("?", 1)[0]
    if request_target != "/version":
        return []

    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""

    status_match = re.match(r"\s*(\d+)", response.status)
    if status_match is None or int(status_match.group(1)) != 200:
        return []

    if '"minimum_compatible_version"' not in body:
        return []

    versions = re.findall(r'"version"\s*:\s*"([0-9a-zA-Z.\-]+)"', body)
    return [finding("rasa-detect", extracted=versions)]
