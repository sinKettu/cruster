metadata = {
    "id": "kubelet-metrics",
    "about": "Kubelet Metrics (passive Nuclei technology fingerprint)",
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
    if path not in ("/", "/metrics"):
        return []

    status = re.match(r"\s*(\d+)", response.status)
    if not status or int(status.group(1)) != 200:
        return []

    try:
        body = base64.b64decode(response.body).decode("latin-1").lower()
    except Exception:
        return []

    markers = [
        "<title>kube metrics server</title>",
        "kube_configmap_",
        "kube_pod_",
        "kube_secret_",
        "kube_deployment_",
        "kube_namespace_",
        "kube_service_",
    ]
    if not any(marker in body for marker in markers):
        return []

    return [finding("kubelet-metrics", extracted=[])]
