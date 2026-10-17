metadata = {
    "id": "localstack-detect",
    "about": "LocalStack - Detect (passive Nuclei technology fingerprint)",
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
    target = request.uri.split("://", 1)[-1]
    target = target[target.find("/") :] if "/" in target else "/"
    target = target.split("#", 1)[0].split("?", 1)[0] or "/"
    if target != "/_localstack/health":
        return []
    status = re.match(r"\s*(\d+)", response.status)
    if not status or int(status.group(1)) != 200:
        return []
    body = base64.b64decode(response.body).decode("latin-1")
    headers = "\n".join(
        header["name"] + ": " + base64.b64decode(header["value"]).decode("latin-1")
        for header in response.headers
    ).lower()
    if '"services"' not in body or "application/json" not in headers:
        return []
    if '"edition"' not in body and '"version"' not in body:
        return []
    extracted = []
    for pattern in (r'"version"\s*:\s*"([^"]+)"', r'"edition"\s*:\s*"([^"]+)"'):
        extracted.extend(match.group(1) for match in re.finditer(pattern, body))
    return [finding("localstack-detect", extracted=extracted)]
