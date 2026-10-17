metadata = {
    "id": "apache-cocoon-detect",
    "about": "Apache Cocoon detect (passive Nuclei technology fingerprint)",
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
    headers = "\n".join(
        item["name"] + ": " + base64.b64decode(item["value"]).decode("latin-1")
        for item in response.headers
    )
    if "X-Cocoon-Version" not in headers:
        return []

    match = re.search(r"X\-Cocoon\-Version:([ 0-9.]+)", headers)
    return [
        finding("apache-cocoon-detect", extracted=[match.group(0)] if match else [])
    ]
