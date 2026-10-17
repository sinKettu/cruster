metadata = {
    "id": "datadog-rum-detect",
    "about": "Datadog RUM - Detect (passive Nuclei technology fingerprint)",
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
    status = re.match(r"\s*(\d+)", response.status)
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    markers = ("datadogRum.init", "DD_RUM.init", "@datadog/browser-rum")
    detected = (
        status
        and int(status.group(1)) == 200
        and any(marker in body for marker in markers)
    )
    return [finding("datadog-rum-detect")] if detected else []
