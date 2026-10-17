metadata = {
    "id": "cloud-run-default-page",
    "about": "Google Cloud Run Default Page (passive Nuclei technology fingerprint)",
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
    del request
    status = re.match(r"\s*(\d+)", response.status)
    body = base64.b64decode(response.body).decode("latin-1")
    if (
        status
        and int(status.group(1)) == 200
        and "Congratulations | Cloud Run" in body
        and 'alt="A group celebrating' in body
    ):
        return [finding("cloud-run-default-page")]
    return []
