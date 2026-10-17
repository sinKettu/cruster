metadata = {
    "id": "flowable-detect",
    "about": "Flowable - Detect (passive Nuclei technology fingerprint)",
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
    if status is None or int(status.group(1)) != 200:
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []
    titles = (
        "<title>Flowable Design</title>",
        "<title>Flowable Control</title>",
        "<title>Flowable</title>",
    )
    markers = (
        "xmlns:flowable=",
        "ng-app=flowableModeler",
        "window.flowable",
        "flowable-logo.png",
    )
    return (
        [finding("flowable-detect")]
        if any(x in body for x in titles) and any(x in body for x in markers)
        else []
    )
