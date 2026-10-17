metadata = {
    "id": "etcd-version",
    "about": "Etcd Version - Detect (passive Nuclei technology fingerprint)",
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
    path = request.uri.split("://", 1)[-1]
    path = path[path.find("/") :] if "/" in path else "/"
    path = path.split("#", 1)[0].split("?", 1)[0] or "/"
    if path != "/version":
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    headers = "\n".join(
        h["name"] + ": " + base64.b64decode(h["value"]).decode("latin-1")
        for h in response.headers
    )
    status = re.match(r"\s*(\d+)", response.status)
    if (
        status
        and int(status.group(1)) == 200
        and '"etcdserver"' in body
        and '"etcdcluster"' in body
        and "application/json" in headers
    ):
        return [finding("etcd-version", extracted=[body.strip("{}")])]
    return []
