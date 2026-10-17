metadata = {
    "id": "tasmoadmin-detect",
    "about": "TasmoAdmin - Detect (passive Nuclei technology fingerprint)",
    "authors": ["ProjectDiscovery contributors", "Cruster"],
    "references": [
        "https://github.com/projectdiscovery/nuclei-templates/tree/main/http/technologies"
    ],
    "tags": ["technology", "discovery", "nuclei", "passive"],
    "severity": "info",
    "type": "passive",
}

import base64

from cruster import HttpRequest, HttpResponse, finding


def scan(request: HttpRequest, response: HttpResponse):
    target = request.uri.split("://", 1)[-1]
    target = target[target.find("/") :] if "/" in target else "/"
    target = target.split("#", 1)[0].split("?", 1)[0] or "/"
    if target != "/login" or response.status.lstrip().split(" ", 1)[0] != "200":
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []
    headers = []
    for item in response.headers:
        try:
            value = base64.b64decode(item["value"]).decode("latin-1")
        except Exception:
            value = ""
        headers.append(item["name"] + ": " + value)
    markers = (
        "<title>Login - TasmoAdmin</title>",
        "/resources/js/compiled/Sonoff.min.js",
    )
    if all(marker in body for marker in markers) and "TASMO_SESSION=" in "\n".join(
        headers
    ):
        return [finding("tasmoadmin-detect")]
    return []
