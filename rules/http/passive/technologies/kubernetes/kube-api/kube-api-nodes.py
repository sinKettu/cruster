metadata = {
    "id": "kube-api-nodes",
    "about": "Kube API Nodes (passive Nuclei technology fingerprint)",
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
    target = target[target.find("/") :] if "/" in target else "/"
    target = target.split("#", 1)[0].split("?", 1)[0] or "/"
    if target != "/api/v1/nodes":
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    node_list = '"NodeList":' in body and '"items":' in body
    version = '"containerRuntimeVersion"' in body and '"kubeletVersion": "v' in body
    if node_list or version:
        return [finding("kube-api-nodes", extracted=[])]
    return []
