metadata = {
    "id": "kener-detect",
    "about": "Kener - Detect (passive Nuclei technology fingerprint)",
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
    target = target.split("#", 1)[0].split("?", 1)[0]
    if target != "/":
        return []

    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""

    status_match = re.match(r"\s*(\d+)", response.status)
    status_code = int(status_match.group(1)) if status_match else 0
    title_pattern = r"(?i)<title>Kener - (?:Open source )?status page system</title>"
    if status_code != 200:
        return []
    if re.search(title_pattern, body) is None:
        return []
    if 'id="kener-app-favicon"' not in body:
        return []
    if 'og:site_name" content="Kener"' not in body:
        return []

    return [finding("kener-detect")]
