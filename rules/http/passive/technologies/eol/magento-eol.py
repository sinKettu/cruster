metadata = {
    "id": "magento-eol",
    "about": "Magento End-of-life Detect (passive Nuclei technology fingerprint)",
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
    body = base64.b64decode(response.body).decode("latin-1")
    match = re.search(r"Magento/([0-9.]+) ", body)
    if match and (tuple(map(int, match.group(1).split("."))) <= (2, 4) or match):
        return [finding("magento-eol")]
    return []
