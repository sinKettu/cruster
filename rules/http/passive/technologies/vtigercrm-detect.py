metadata = {
    "id": "vtigercrm-detect",
    "about": "Vtiger CRM - Detect (passive Nuclei technology fingerprint)",
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
    if not response.status.startswith("200"):
        return []
    if "Powered by vtiger CRM" not in body or "vtiger" not in body:
        return []
    pattern = r"Powered by vtiger CRM \- ([0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3})"
    extracted = [match.group(1) for match in re.finditer(pattern, body)]
    return [finding("vtigercrm-detect", extracted=extracted)]
