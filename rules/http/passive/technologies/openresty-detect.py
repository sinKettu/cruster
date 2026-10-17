metadata = {
    "id": "openresty-detect",
    "about": "OpenResty detection (passive Nuclei technology fingerprint)",
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
    headers = []
    server_values = []
    for item in response.headers:
        try:
            value = base64.b64decode(item["value"]).decode("latin-1")
        except Exception:
            value = ""
        headers.append(item["name"] + ": " + value)
        if item["name"].lower().replace("-", "_") == "server":
            server_values.append(value)
    if "openresty" not in "\n".join(server_values):
        return []
    versions = re.findall(r"openresty/[0-9.]+", "\n".join(headers))
    return [finding("openresty-detect", extracted=versions)]
