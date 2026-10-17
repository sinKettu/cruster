metadata = {
    "id": "zk-framework-detect",
    "about": "ZK Framework - Detect (passive Nuclei technology fingerprint)",
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
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []

    markers = (
        "ZK Framework",
        "zk.wpd",
        "zkau/web",
        "zkau/zkau",
        "zkau/web/js/zk/zk.wpd",
        "<!-- ZK ",
    )
    patterns = (
        r"ZK Framework v?([0-9.]+)",
        r"<!-- ZK v?([0-9.]+) [\d\s\w]+ -->",
        r"zkau/web/js/zk/zk\.wpd\?v=([0-9.]+)",
        r"zkau/web/js/zk/zk\.js\?v=([0-9.]+)",
    )
    versions = []
    regex_matched = False
    for pattern in patterns:
        matches = list(re.finditer(pattern, body))
        if matches:
            regex_matched = True
        versions.extend(match.group(1) for match in matches)

    word_matched = any(marker in body for marker in markers)
    if not word_matched and not regex_matched:
        return []

    return [finding("zk-framework-detect", extracted=versions)]
