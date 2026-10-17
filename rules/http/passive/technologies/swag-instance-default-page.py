metadata = {
    "id": "swag-instance-default-page",
    "about": "Swag Instance Default Page Panel - Detect (passive Nuclei technology fingerprint)",
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
    try:
        body = base64.b64decode(response.body).decode("latin-1")
        headers = "\n".join(
            item["name"] + ": " + base64.b64decode(item["value"]).decode("latin-1")
            for item in response.headers
        )
    except Exception:
        body = ""
        headers = ""
    status = re.match(r"\s*(\d+)", response.status)
    matched = (
        status is not None
        and int(status.group(1)) == 200
        and "<title>Welcome to your SWAG instance</title>" in body
        and "text/html" in headers
    )
    return [finding("swag-instance-default-page", extracted=[])] if matched else []
