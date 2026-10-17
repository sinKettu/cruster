metadata = {
    "id": "puppetdb-detect",
    "about": "PuppetDB Detection (passive Nuclei technology fingerprint)",
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
    if target != "/pdb/meta/v1/version":
        return []
    headers = []
    for item in response.headers:
        try:
            headers.append(
                item["name"] + ": " + base64.b64decode(item["value"]).decode("latin-1")
            )
        except Exception:
            headers.append(item["name"] + ": ")
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    versions = [
        match.group(1) for match in re.finditer(r'"version"\s:\s"([0-9.]+)"', body)
    ]
    status = re.match(r"\s*(\d+)", response.status)
    detected = (
        status is not None
        and int(status.group(1)) == 200
        and "application/json" in "\n".join(headers)
        and re.search(r'"version"\s:\s"([0-9.]+)"', body) is not None
    )
    return [finding("puppetdb-detect", extracted=versions)] if detected else []
