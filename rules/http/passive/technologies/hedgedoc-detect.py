metadata = {
    "id": "hedgedoc-detect",
    "about": "HedgeDoc Collaborative Editor - Detect (passive Nuclei technology fingerprint)",
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
    target = request.uri.split("://", 1)[-1]
    target = target[target.find("/") :] if "/" in target else "/"
    target = target.split("#", 1)[0].split("?", 1)[0] or "/"
    if target != "/":
        return []

    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []

    header_lines = []
    for header in response.headers:
        try:
            value = base64.b64decode(header["value"]).decode("latin-1")
        except Exception:
            return []
        header_lines.append(header["name"] + ": " + value)
    headers = "\n".join(header_lines)

    header_match = "hedgedoc-version:" in headers.lower()
    body_match = "HedgeDoc - Ideas" in body
    body_match = (
        body_match or 'Powered by <a href="https://hedgedoc.org">HedgeDoc' in body
    )
    if not header_match and not body_match:
        return []

    versions = re.findall(r"(?i)hedgedoc-version:\s*([0-9.]+)", headers)
    return [finding("hedgedoc-detect", extracted=versions)]
