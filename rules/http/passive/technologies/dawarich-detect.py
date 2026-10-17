metadata = {
    "id": "dawarich-detect",
    "about": "Dawarich - Detect (passive Nuclei technology fingerprint)",
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
    if target != "/api/v1/health":
        return []
    status = re.match(r"\s*(\d+)", response.status)
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    version = ""
    header = []
    for item in response.headers:
        try:
            value = base64.b64decode(item["value"]).decode("latin-1")
        except Exception:
            value = ""
        header.append(item["name"] + ": " + value)
        if item["name"].lower().replace("-", "_") == "x_dawarich_version":
            version = value
    detected = (
        status
        and int(status.group(1)) == 200
        and "X-Dawarich-Version" in "\n".join(header)
        and '"status":"ok"' in body
    )
    return (
        [finding("dawarich-detect", extracted=[version] if version else [])]
        if detected
        else []
    )
