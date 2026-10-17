metadata = {
    "id": "livehelperchat-detect",
    "about": "Live Helper Chat Detect (passive Nuclei technology fingerprint)",
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
    patterns = (r"(?i)Live.Helper.Chat", r"(?i)Copyright.\(c\).2019.Remigijus.Kiminas")
    if not any(re.search(pattern, body) for pattern in patterns):
        return []
    version_pattern = r"(?i)^((\d+\.)?(\d+\.)?(\*|\d+))v"
    versions = [match.group(1) for match in re.finditer(version_pattern, body)]
    return [finding("livehelperchat-detect", extracted=versions)]
