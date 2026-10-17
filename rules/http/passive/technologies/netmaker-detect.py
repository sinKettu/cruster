metadata = {
    "id": "netmaker-detect",
    "about": "Netmaker - Detect (passive Nuclei technology fingerprint)",
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
    body = base64.b64decode(response.body).decode("latin-1")
    status = re.match(r"\s*(\d+)", response.status)
    secondary = (
        'id="nmui-loading"',
        "The management UI for Netmaker. Netmaker makes networks.",
    )
    if (
        status
        and int(status.group(1)) == 200
        and "<title>Netmaker</title>" in body
        and any(value in body for value in secondary)
    ):
        return [finding("netmaker-detect")]
    return []
