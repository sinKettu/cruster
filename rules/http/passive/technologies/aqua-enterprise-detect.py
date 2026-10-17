metadata = {
    "id": "aqua-enterprise-detect",
    "about": "Aqua Enterprise Detect (passive Nuclei technology fingerprint)",
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
    target = target.split("?", 1)[0].split("#", 1)[0]
    status = re.match(r"\s*(\d+)", response.status)
    if target != "/api" or status is None or int(status.group(1)) != 200:
        return []

    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []

    markers = ('"version"', '"enable_sso"', '"num_protected_kube_nodes"')
    if not all(marker in body for marker in markers):
        return []

    versions = [
        match.group(1) for match in re.finditer(r'\{"version":"([0-9.]+)",', body)
    ]
    return [finding("aqua-enterprise-detect", extracted=versions)]
