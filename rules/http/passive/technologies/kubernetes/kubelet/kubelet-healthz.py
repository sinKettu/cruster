metadata = {
    "id": "kubelet-healthz",
    "about": "Kubelet Healthz (passive Nuclei technology fingerprint)",
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
    path = target[target.find("/") :] if "/" in target else "/"
    path = path.split("#", 1)[0].split("?", 1)[0] or "/"
    if path != "/healthz":
        return []

    status = re.match(r"\s*(\d+)", response.status)
    if not status or int(status.group(1)) != 200:
        return []

    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []

    header_lines = []
    for header in response.headers:
        try:
            value = base64.b64decode(header["value"]).decode("latin-1")
        except Exception:
            continue
        header_lines.append(header["name"] + ": " + value)

    headers = "\n".join(header_lines)
    if "text/plain" not in headers or "ok" not in body or "Kubernetes" not in headers:
        return []

    return [finding("kubelet-healthz", extracted=[])]
