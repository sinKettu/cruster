metadata = {
    "id": "apache-ofbiz-detect",
    "about": "Apache OFBiz - Detect (passive Nuclei technology fingerprint)",
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
    if not response.status.startswith("200"):
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""

    headers = "\n".join(
        item["name"] + ": " + base64.b64decode(item["value"]).decode("latin-1")
        for item in response.headers
    )
    text = headers + "\n\n" + body
    if "OFBiz.Visitor=" not in text and "Apache OFBiz." not in text:
        return []

    return [finding("apache-ofbiz-detect")]
