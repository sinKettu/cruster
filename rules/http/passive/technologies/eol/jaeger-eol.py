metadata = {
    "id": "jaeger-eol",
    "about": "Jaeger End-of-Life - Detect (passive Nuclei technology fingerprint)",
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
    body = base64.b64decode(response.body).decode("latin-1")
    versions = [
        match.group(1)
        for match in re.finditer(r'"gitVersion":"v?([0-9]+\.[0-9.]+)"', body)
    ]
    if versions and tuple(map(int, versions[0].split("."))) < (2, 0):
        return [finding("jaeger-eol", extracted=versions)]
    return []
