metadata = {
    "id": "bamboo-detect",
    "about": "Bamboo - Detection (passive Nuclei technology fingerprint)",
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
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []

    lowered = body.lower()
    detected = (
        "<title>log into atlassian - atlassian bamboo</title>" in lowered
        or 'meta name="application-name" content="bamboo" />' in lowered
    )
    if not detected:
        return []

    versions = [
        match.group(1)
        for match in re.finditer(r"(?i)atlassian bamboo</a> version (.*) -", body)
    ]
    return [finding("bamboo-detect", extracted=versions)]
