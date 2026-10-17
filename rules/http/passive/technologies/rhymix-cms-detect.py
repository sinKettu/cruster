metadata = {
    "id": "rhymix-cms-detect",
    "about": "Rhymix CMS Detect (passive Nuclei technology fingerprint)",
    "authors": ["ProjectDiscovery contributors", "Cruster"],
    "references": [
        "https://github.com/projectdiscovery/nuclei-templates/tree/main/http/technologies"
    ],
    "tags": ["technology", "discovery", "nuclei", "passive"],
    "severity": "info",
    "type": "passive",
}

import base64

from cruster import HttpRequest, HttpResponse, finding


def scan(request: HttpRequest, response: HttpResponse):
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""

    if '<meta name="generator" content="Rhymix" />' in body:
        return [finding("rhymix-cms-detect", extracted=[])]

    header_lines = []
    for header in response.headers:
        try:
            value = base64.b64decode(header["value"]).decode("latin-1")
        except Exception:
            value = ""
        header_lines.append(header["name"] + ": " + value)

    if "rx_sesskey1" not in "\n".join(header_lines):
        return []

    return [finding("rhymix-cms-detect", extracted=[])]
