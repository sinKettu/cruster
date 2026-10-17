metadata = {
    "id": "salesforce-b2c-commerce-webdav",
    "about": "Salesforce B2C Commerce WebDAV - Detection (passive Nuclei technology fingerprint)",
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
    except Exception:
        body = ""

    status_match = re.match(r"\s*(\d+)", response.status)
    status_code = int(status_match.group(1)) if status_match else 0
    required_words = ("/waroot/style.css", "/waroot/system_arrow.gif")
    if status_code != 404 or not all(word in body for word in required_words):
        return []

    return [finding("salesforce-b2c-commerce-webdav", extracted=[])]
