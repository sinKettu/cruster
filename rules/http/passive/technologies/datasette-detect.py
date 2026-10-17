metadata = {
    "id": "datasette-detect",
    "about": "Datasette - Detect (passive Nuclei technology fingerprint)",
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
    if (
        status
        and int(status.group(1)) == 200
        and "<title>Datasette:" in body
        and 'href="https://datasette.io/" title="Datasette v' in body
    ):
        version = re.search(
            r'href="https://datasette\.io/" title="Datasette v([^" ]+)', body
        )
        return [
            finding("datasette-detect", extracted=[version.group(1)] if version else [])
        ]
    return []
