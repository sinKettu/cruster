metadata = {
    "id": "grav-cms-detect",
    "about": "Grav CMS Detect (passive Nuclei technology fingerprint)",
    "authors": ["ProjectDiscovery contributors", "Cruster"],
    "references": [
        "https://github.com/projectdiscovery/nuclei-templates/tree/main/http/technologies"
    ],
    "tags": ["technology", "discovery", "nuclei", "passive"],
    "severity": "info",
    "type": "passive"
}

import base64

from cruster import HttpRequest, HttpResponse, finding


def scan(request: HttpRequest, response: HttpResponse):
    del request
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""

    generator_marker = '<meta name="generator" content="GravCMS" />'
    if generator_marker in body:
        return [finding("grav-cms-detect")]

    for header in response.headers:
        try:
            value = base64.b64decode(header["value"]).decode("latin-1")
        except Exception:
            continue

        header_line = header["name"] + ": " + value
        if "Set-Cookie: grav-site-" in header_line:
            return [finding("grav-cms-detect")]

    return []
