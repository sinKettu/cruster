metadata = {
    "id": "django-detect",
    "about": "Django Detection (passive Nuclei technology fingerprint)",
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
    del request
    status = re.match(r"\s*(\d+)", response.status)
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    markers = ("powered by django", "django-project", "django software foundation")
    if (
        not status
        or int(status.group(1)) not in (200, 403, 404)
        or not any(marker in body.lower() for marker in markers)
    ):
        return []
    evidence = []
    for pattern in (
        r"Django ([0-9.]+)",
        r"Powered by Django ([0-9.]+)",
        r"django-site-id=([a-z0-9]+)",
        r"django\.org/([^/]+)",
    ):
        evidence.extend(
            match.group(1) for match in re.finditer(pattern, body, re.IGNORECASE)
        )
    return [finding("django-detect", extracted=list(dict.fromkeys(evidence)))]
