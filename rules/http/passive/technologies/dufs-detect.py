metadata = {
    "id": "dufs-detect",
    "about": "DUFS - Detect (passive Nuclei technology fingerprint)",
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
    del request
    status = re.match(r"\s*(\d+)", response.status)
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    if (
        not status
        or int(status.group(1)) != 200
        or not all(x in body.lower() for x in ("__dufs_v", "allow_upload"))
    ):
        return []
    version = re.search(r"(?i)__dufs_v([0-9.]+)_", body)
    return [finding("dufs-detect", extracted=[version.group(1)] if version else [])]
