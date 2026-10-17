metadata = {
    "id": "openbas-detect",
    "about": "OpenBAS - Detect (passive Nuclei technology fingerprint)",
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
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    status = re.match(r"\s*(\d+)", response.status)
    if (
        status
        and int(status.group(1)) == 200
        and "<title>OpenBAS - Open Breach & Attack Simulation Platform</title>" in body
        and "OpenBAS is an open source platform allowing organizations to plan, "
        "schedule and conduct adversary simulation campaigns" in body
    ):
        return [finding("openbas-detect", extracted=[])]
    return []
