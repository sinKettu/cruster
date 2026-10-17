metadata = {
    "id": "kodexplorer-detect",
    "about": "KodExplorer-detect (passive Nuclei technology fingerprint)",
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
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""

    title_pattern = r"(?i)<title>.* - Powered by KodExplorer</title>"
    if re.search(title_pattern, body) is None:
        return []

    version_pattern = r"Powered by KodExplorer ([a-z0-9.]+) \|"
    versions = [match.group(1) for match in re.finditer(version_pattern, body)]
    return [finding("kodexplorer-detect", extracted=versions)]
