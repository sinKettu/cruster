metadata = {
    "id": "hashicorp-vault-detect",
    "about": "HashiCorp Vault Detect (passive Nuclei technology fingerprint)",
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
    if target not in ("/v1/sys/seal-status", "/ui/vault/auth"):
        return []

    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []

    if '<meta name="vault/config/environment"' in body:
        return [finding("hashicorp-vault-detect")]

    status_match = re.match(r"\s*(\d+)", response.status)
    if status_match is None or int(status_match.group(1)) != 200:
        return []

    content_types = []
    for header in response.headers:
        if header["name"].lower().replace("-", "_") != "content_type":
            continue
        try:
            content_types.append(base64.b64decode(header["value"]).decode("latin-1"))
        except Exception:
            return []
    if "application/json" not in "\n".join(content_types):
        return []

    markers = ('sealed":', 'cluster_id":', "recovery_seal_type")
    if not all(marker in body for marker in markers):
        return []

    return [finding("hashicorp-vault-detect")]
