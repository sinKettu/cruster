import base64
import re

from cruster import HttpRequest, HttpResponse, finding

metadata = {
    "id": "prestashop-detect",
    "about": "PrestaShop Tech Detection (passive Nuclei technology fingerprint)",
    "authors": ["ProjectDiscovery contributors", "Cruster"],
    "references": [
        "https://github.com/projectdiscovery/nuclei-templates/tree/main/http/technologies",
    ],
    "tags": ["technology", "discovery", "nuclei", "passive"],
    "severity": "info",
    "type": "passive",
}


PATTERNS = (
    r"Powered by <a\s+[^>]+>PrestaShop",
    r"<!-- /Block [a-z ]+ module (?:HEADER|TOP)?\s?-->",
    r"<!-- /Module Block [a-z ]+ -->",
)


def scan(request: HttpRequest, response: HttpResponse):
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    detected = any(re.search(pattern, body) is not None for pattern in PATTERNS)
    return [finding("prestashop-detect", extracted=[])] if detected else []
