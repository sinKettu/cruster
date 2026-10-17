metadata = {
    "id": "openshift-detect",
    "about": "OpenShift Monitoring Plugin - Detect (passive Nuclei technology fingerprint)",
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
    path = request.uri.split("://", 1)[-1]
    path = path[path.find("/") :] if "/" in path else "/"
    path = path.split("#", 1)[0].split("?", 1)[0] or "/"
    if path != "/plugin-manifest.json":
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    status = re.match(r"\s*(\d+)", response.status)
    if not (
        status
        and int(status.group(1)) == 200
        and "openshift console monitoring plugin" in body.lower()
    ):
        return []
    versions = re.findall(r'"version"\s*:\s*"([0-9\.]+)"', body)
    return [finding("openshift-detect", extracted=versions)]
