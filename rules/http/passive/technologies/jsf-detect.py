metadata = {
    "id": "jsf-detect",
    "about": "JavaServer Faces Detection (passive Nuclei technology fingerprint)",
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
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""

    has_javafaces = "javax.faces.resource" in body or "javax.faces.ViewState" in body
    evidence = []
    if has_javafaces:
        evidence.append("javafaces")
    if "primefaces" in body and has_javafaces:
        evidence.append("primefaces")
    if "richfaces" in body and has_javafaces:
        evidence.append("richfaces")
    if not evidence:
        return []

    return [finding("jsf-detect", extracted=evidence)]
