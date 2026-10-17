metadata = {
    "id": "vmware-vrealize",
    "about": "VMware vRealize (passive Nuclei technology fingerprint)",
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
    target = request.uri.split("://", 1)[-1]
    target = target[target.find("/") :] if "/" in target else "/"
    target = target.split("#", 1)[0].split("?", 1)[0] or "/"
    status = re.match(r"\s*(\d+)", response.status)
    body = base64.b64decode(response.body).decode("latin-1")
    headers = "\n".join(
        item["name"] + ": " + base64.b64decode(item["value"]).decode("latin-1")
        for item in response.headers
    )
    versions = [
        match.group(1)
        for match in re.finditer(r"SessionProvider.js\?version=([0-9.]+)", body)
    ]
    if (
        target == "/ui/login.action"
        and status
        and int(status.group(1)) == 200
        and "text/html" in headers
        and "<title>vRealize Operations Manager</title>" in body
    ):
        return [finding("vmware-vrealize", extracted=versions)]
    return []
