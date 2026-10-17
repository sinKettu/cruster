metadata = {
    "id": "weberp-detect",
    "about": "webERP - Detect (passive Nuclei technology fingerprint)",
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


def scan(_request: HttpRequest, response: HttpResponse):
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []
    fields = ['name="CompanyNameField"', 'name="UserNameEntryField"']
    if not response.status.startswith("200"):
        return []
    if re.search(r"(?i)<title>webERP Login screen</title>", body) is None:
        return []
    if not all(field in body for field in fields):
        return []
    return [finding("weberp-detect")]
