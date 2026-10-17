metadata = {
    "id": "apache-hive-detect",
    "about": "Apache Hive - Detect (passive Nuclei technology fingerprint)",
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
    if not response.status.startswith("200"):
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""

    markers = ("<title>HiveServer2</title>", "hiveserver2")
    if not all(marker in body for marker in markers):
        return []

    match = re.search(
        r"<td>([0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}), \S+</td>\n\s+<td>Hive version and revision</td>",
        body,
    )
    return [finding("apache-hive-detect", extracted=[match.group(1)] if match else [])]
