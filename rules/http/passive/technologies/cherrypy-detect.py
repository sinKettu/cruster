metadata = {
    "id": "cherrypy-detect",
    "about": "CherryPy Web Server - Detect (passive Nuclei technology fingerprint)",
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
    headers = []
    for item in response.headers:
        try:
            value = base64.b64decode(item["value"]).decode("latin-1")
        except Exception:
            value = ""
        headers.append(item["name"] + ": " + value)
    header = "\n".join(headers)
    if "Server: CherryPy" not in header:
        return []
    extracted = [match.group(1) for match in re.finditer(r"CherryPy/([0-9.]+)", header)]
    return [finding("cherrypy-detect", extracted=extracted)]
