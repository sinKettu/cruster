metadata = {
    "id": "kube-api-secrets",
    "about": "Kube API Secrets (passive Nuclei technology fingerprint)",
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
    target = request.uri.split("://", 1)[-1]
    target = target[target.find("/") :] if "/" in target else "/"
    target = target.split("#", 1)[0].split("?", 1)[0] or "/"
    if target != "/api/v1/namespaces/default/secrets":
        return []
    status = re.match(r"\s*(\d+)", response.status)
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    if (
        status
        and int(status.group(1)) == 200
        and '"SecretList":' in body
        and '"items":' in body
    ):
        return [finding("kube-api-secrets", extracted=[])]
    return []
