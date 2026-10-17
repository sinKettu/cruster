metadata = {
    "id": "php-eol",
    "about": "PHP End-of-Life - Detect (passive Nuclei technology fingerprint)",
    "authors": [
        "ProjectDiscovery contributors",
        "Cruster"
    ],
    "references": [
        "https://github.com/projectdiscovery/nuclei-templates/tree/main/http/technologies"
    ],
    "tags": [
        "technology",
        "discovery",
        "nuclei",
        "passive"
    ],
    "severity": "info",
    "type": "passive"
}

import base64
import re

from cruster import HttpRequest, HttpResponse, finding


def scan(request: HttpRequest, response: HttpResponse):
    del request
    headers = "\n".join(
        item["name"] + ": " + base64.b64decode(item["value"]).decode("latin-1")
        for item in response.headers
    )
    versions = []
    patterns = (
        r"X-Powered-By: PHP/([0-9.]+)",
        r"Server:.*PHP/([0-9.]+)",
    )
    for pattern in patterns:
        versions.extend(match.group(1) for match in re.finditer(pattern, headers))
    is_eol = (
        versions
        and tuple(map(int, versions[0].split("."))) < (8, 0, 0)
        and "php" in headers.lower()
    )
    if is_eol:
        return [finding("php-eol", extracted=list(dict.fromkeys(versions)))]
    return []
