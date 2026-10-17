metadata = {
    "id": "meteobridge-detect",
    "about": "MeteoBridge - Detect (passive Nuclei technology fingerprint)",
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
    target = target.split("#", 1)[0].split("?", 1)[0] or "/"
    if target != "/cgi-bin/meteobridge.cgi":
        return []
    headers = "\n".join(
        item["name"] + ": " + base64.b64decode(item["value"]).decode("latin-1")
        for item in response.headers
    )
    status = re.match(r"\s*(\d+)", response.status)
    if (
        status
        and int(status.group(1)) == 401
        and 'Basic realm="MeteoBridge"' in headers
    ):
        return [finding("meteobridge-detect")]
    return []
