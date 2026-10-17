import base64
import re

from cruster import HttpRequest, HttpResponse, finding

metadata = {
    "id": "projectsend-detect",
    "about": "ProjectSend Login Panel - Detect (passive Nuclei technology fingerprint)",
    "authors": ["ProjectDiscovery contributors", "Cruster"],
    "references": [
        "https://github.com/projectdiscovery/nuclei-templates/tree/main/http/technologies"
    ],
    "tags": ["technology", "discovery", "nuclei", "passive"],
    "severity": "info",
    "type": "passive",
}


def scan(request: HttpRequest, response: HttpResponse):
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    versions = [
        match.group(1) for match in re.finditer(r"version (.*) - Free software", body)
    ]
    status = re.match(r"\s*(\d+)", response.status)
    signature = r'Provided by <a href="https?:\/\/www\.projectsend\.org\/" target="_blank">ProjectSend<\/a>'
    detected = (
        status is not None
        and int(status.group(1)) == 200
        and re.search(signature, body) is not None
    )
    return [finding("projectsend-detect", extracted=versions)] if detected else []
