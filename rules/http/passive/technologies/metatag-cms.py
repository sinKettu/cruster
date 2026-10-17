metadata = {
    "id": "metatag-cms",
    "about": "Metatag CMS Detection (passive Nuclei technology fingerprint)",
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
    del request
    body = base64.b64decode(response.body).decode("latin-1")
    headers = "\n".join(
        item["name"] + ": " + base64.b64decode(item["value"]).decode("latin-1")
        for item in response.headers
    )
    pattern = r'(?i)<meta\s+?name="?generator"?\s+?content="([^"]+?)"'
    matches = [match.group(1) for match in re.finditer(pattern, body)]
    if "text/html" in headers and matches:
        return [finding("metatag-cms", extracted=matches)]
    return []
