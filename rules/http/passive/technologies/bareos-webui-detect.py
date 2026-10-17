metadata = {
    "id": "bareos-webui-detect",
    "about": "Bareos WebUI - Detect (passive Nuclei technology fingerprint)",
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
    status = re.match(r"\s*(\d+)", response.status)
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []

    detected = (
        status
        and int(status.group(1)) == 200
        and "<title>Bareos - Login</title>" in body
        and "img/bareos.png" in body
    )
    return [finding("bareos-webui-detect")] if detected else []
