metadata = {
    "id": "jan-detect",
    "about": "Jan AI - Detect (passive Nuclei technology fingerprint)",
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


def _request_path(uri):
    value = uri.split("://", 1)[-1]
    value = value[value.find("/") :] if "/" in value else "/"
    return value.split("#", 1)[0].split("?", 1)[0] or "/"


def scan(request: HttpRequest, response: HttpResponse):
    if _request_path(request.uri) not in ("/v1/models", "/healthz"):
        return []

    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""

    content_type = ""
    for header in response.headers:
        if header["name"].lower() != "content-type":
            continue
        try:
            content_type = base64.b64decode(header["value"]).decode("latin-1")
        except Exception:
            content_type = ""
        break

    status = re.match(r"\s*(\d+)", response.status)
    if not status or int(status.group(1)) != 200:
        return []
    if "application/json" not in content_type:
        return []
    if not (
        ('"object":"list"' in body and '"object":"model"' in body)
        or "ok" in body.lower()
    ):
        return []

    return [finding("jan-detect")]
