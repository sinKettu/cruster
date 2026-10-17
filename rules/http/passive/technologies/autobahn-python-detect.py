metadata = {
    "id": "autobahn-python-detect",
    "about": "Autobahn-Python Webserver Detect (passive Nuclei technology fingerprint)",
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
    status = re.match(r"\s*(\d+)", response.status)
    if status is None or int(status.group(1)) != 200:
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []

    if re.search(r"<h1>AutobahnPython([ 0-9.]+)</h1>", body) is None:
        return []

    extracted = [
        match.group(0) for match in re.finditer(r"AutobahnPython([ 0-9.]+)", body)
    ]
    return [finding("autobahn-python-detect", extracted=extracted)]
