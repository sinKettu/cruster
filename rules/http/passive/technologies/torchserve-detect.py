metadata = {
    "id": "torchserve-detect",
    "about": "PyTorch TorchServe - Detect (passive Nuclei technology fingerprint)",
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
    target = request.uri.split("://", 1)[-1]
    if "/" in target:
        target = target[target.find("/") :]
    else:
        target = "/"
    target = target.split("#", 1)[0].split("?", 1)[0]
    if target not in ("/api-description", "/models", "/ping"):
        return []
    status_code = response.status.lstrip().split(" ", 1)[0]
    if status_code != "200":
        return []

    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []

    content_types = []
    for header in response.headers:
        if header["name"].lower() != "content-type":
            continue
        try:
            content_types.append(base64.b64decode(header["value"]).decode("latin-1"))
        except Exception:
            continue
    if not any("application/json" in value for value in content_types):
        return []

    api_description = any(
        indicator in body
        for indicator in (
            '"TorchServe APIs"',
            '"apiDescription"',
            '"operationId"',
        )
    )
    models = '"models"' in body and '"modelName"' in body
    healthy = '"status"' in body and "Healthy" in body
    if not (api_description or models or healthy):
        return []
    return [finding("torchserve-detect")]
