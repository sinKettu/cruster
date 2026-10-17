metadata = {
    "id": "4D-detect",
    "about": "4D Detection (passive Nuclei technology fingerprint)",
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
    target = request.uri.split("://", 1)[-1]
    target = target[target.find("/") :] if "/" in target else "/"
    target = target.split("#", 1)[0].split("?", 1)[0]
    status = re.match(r"\s*(\d+)", response.status)
    if target != "/" or status is None or int(status.group(1)) != 200:
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1")
        headers = "\n".join(
            item["name"] + ": " + base64.b64decode(item["value"]).decode("latin-1")
            for item in response.headers
        )
    except Exception:
        return []
    if not any(
        word in body
        for word in (
            "/4DACTION/",
            "/4DSTATS",
            "/4DHTMLSTATS",
            "/4DCACHECLEAR",
            "/4DWEBTEST",
        )
    ):
        return []
    versions = [
        match.group(1)
        for match in re.finditer(r"Server:\s+4D(?:_V[0-9]+)?/([0-9.]+)", headers)
    ]
    return [finding("4D-detect", extracted=versions)]
