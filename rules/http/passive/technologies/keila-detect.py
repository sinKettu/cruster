metadata = {
    "id": "keila-detect",
    "about": "Keila - Detect (passive Nuclei technology fingerprint)",
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
    if target != "/":
        return []

    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""

    status_match = re.match(r"\s*(\d+)", response.status)
    status_code = int(status_match.group(1)) if status_match else 0
    title_pattern = r"(?is)<title>\s*Sign in\s*·\s*Keila\s*</title>"
    if status_code != 200:
        return []
    if re.search(title_pattern, body) is None:
        return []
    if 'content="Open Source Newsletters"' not in body:
        return []
    if 'placeholder="you@keila.io"' not in body:
        return []

    return [finding("keila-detect")]
