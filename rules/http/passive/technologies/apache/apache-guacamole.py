metadata = {
    "id": "apache-guacamole",
    "about": "Apache Guacamole Login Page and version detection (passive Nuclei technology fingerprint)",
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
    path = request.uri.split("://", 1)[-1]
    path = path[path.find("/") :] if "/" in path else "/"
    path = path.split("#", 1)[0].split("?", 1)[0] or "/"
    if path != "/translations/en.json" or not response.status.startswith("200"):
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""

    if "Apache Guacamole" not in body:
        return []

    match = re.search(r'"VERSION":"[0-9]+\.2\.0"', body)
    return [finding("apache-guacamole", extracted=[match.group(0)] if match else [])]
