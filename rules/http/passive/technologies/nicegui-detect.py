metadata = {
    "id": "nicegui-detect",
    "about": "NiceGUI Detection (passive Nuclei technology fingerprint)",
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
    headers = "\n".join(
        item["name"] + ": " + base64.b64decode(item["value"]).decode("latin-1")
        for item in response.headers
    )
    if "x-nicegui-content" not in headers.lower():
        return []

    body = base64.b64decode(response.body).decode("latin-1")
    versions = re.findall(r"/_nicegui/([0-9.]+)/", body)
    return [finding("nicegui-detect", extracted=versions)]
