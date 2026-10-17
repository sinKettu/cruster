metadata = {
    "id": "terraform-detect",
    "about": "Detect Terraform Provider (passive Nuclei technology fingerprint)",
    "authors": ["ProjectDiscovery contributors", "Cruster"],
    "references": [
        "https://github.com/projectdiscovery/nuclei-templates/tree/main/http/technologies"
    ],
    "tags": ["technology", "discovery", "nuclei", "passive"],
    "severity": "info",
    "type": "passive"
}

import base64

from cruster import HttpRequest, HttpResponse, finding


def scan(request: HttpRequest, response: HttpResponse):
    target = request.uri.split("://", 1)[-1]
    if "/" in target:
        target = target[target.find("/") :]
    else:
        target = "/"
    target = target.split("#", 1)[0].split("?", 1)[0]
    if target != "/provider.tf":
        return []
    status_code = response.status.lstrip().split(" ", 1)[0]
    if status_code != "200":
        return []

    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []

    if "access_key" not in body or "terraform" not in body:
        return []
    return [finding("terraform-detect")]
