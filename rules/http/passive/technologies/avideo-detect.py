metadata = {
    "id": "avideo-detect",
    "about": "Avideo Detect (passive Nuclei technology fingerprint)",
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
    status = re.match(r"\s*(\d+)", response.status)
    if status is None or int(status.group(1)) != 200:
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1")
        headers = "\n".join(
            header["name"] + ": " + base64.b64decode(header["value"]).decode("latin-1")
            for header in response.headers
        )
    except Exception:
        return []

    detected = re.search(r"<title>(.*)AVideo(.*)</title>", body) is not None
    if not detected and "AVideo Analytics" not in body:
        return []
    if "/install/index.php" in headers:
        return []

    return [finding("avideo-detect")]
