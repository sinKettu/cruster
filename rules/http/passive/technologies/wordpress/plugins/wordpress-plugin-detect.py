metadata = {
    "id": "wordpress-plugin-detect",
    "about": "Wordpress Plugin Detection (passive Nuclei technology fingerprint)",
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
    status = re.match(r"\s*(\d+)", response.status)
    if status is None or int(status.group(1)) != 200:
        return []

    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []

    plugins = sorted(set(re.findall(r"/plugins/([A-Za-z0-9._-]+)/", body)))
    if not plugins:
        return []

    return [finding("wordpress-plugin-detect", extracted=plugins)]
