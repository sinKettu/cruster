metadata = {
    "id": "flowfuse-detect",
    "about": "FlowFuse - Detect (passive Nuclei technology fingerprint)",
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
    if status is None or int(status.group(1)) != 200:
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []
    titles = (
        "<title>FlowFuse</title>",
        "<title>Log in - FlowFuse</title>",
        "<title>FlowFuse Dashboard</title>",
    )
    markers = (
        "FlowFuse doesn't work properly without JavaScript enabled",
        'meta name="description" content="FlowFuse Dashboard"',
        "theme&#x2F;css&#x2F;forge-common.css",
    )
    return (
        [finding("flowfuse-detect")]
        if any(x in body for x in titles) and any(x in body for x in markers)
        else []
    )
