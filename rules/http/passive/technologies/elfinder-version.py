metadata = {
    "id": "elfinder-version",
    "about": "elFinder Version - Detect (passive Nuclei technology fingerprint)",
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
    if target not in ("/js/elfinder.min.js", "/js/elFinder.version.js"):
        return []
    status = re.match(r"\s*(\d+)", response.status)
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    if (
        not status
        or int(status.group(1)) != 200
        or not (
            "elFinder - file manager for web" in body
            or "elFinder.prototype.version =" in body
        )
    ):
        return []
    evidence = []
    for pattern in (
        r"\* Version (.+) \(",
        r"elFinder\.prototype\.version = '([0-9.]+)';",
    ):
        evidence.extend(match.group(1) for match in re.finditer(pattern, body))
    return [finding("elfinder-version", extracted=list(dict.fromkeys(evidence)))]
