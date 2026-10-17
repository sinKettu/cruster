metadata = {
    "id": "backdrop-cms-detect",
    "about": "Backdrop CMS - Detect (passive Nuclei technology fingerprint)",
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
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""

    headers = []
    for item in response.headers:
        try:
            value = base64.b64decode(item["value"]).decode("latin-1")
        except Exception:
            value = ""
        headers.append(item["name"] + ": " + value)

    complete_response = "\n".join(headers) + "\n\n" + body
    if not (
        "Added by Backdrop CMS packaging script" in complete_response
        or "X-Generator: Backdrop CMS" in complete_response
    ):
        return []

    versions = [match.group(0) for match in re.finditer(r"version = ([0-9.x-]+)", body)]
    return [finding("backdrop-cms-detect", extracted=versions)]
