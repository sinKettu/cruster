metadata = {
    "id": "apache-nifi-detect",
    "about": "Apache NiFi detect (passive Nuclei technology fingerprint)",
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
    target = request.uri.split("://", 1)[-1]
    target = target[target.find("/") :] if "/" in target else "/"
    target = target.split("#", 1)[0].split("?", 1)[0]
    paths = (
        "/access/config",
        "/controller/config",
        "/controller/registry-clients",
        "/counters",
        "/flow/registries",
        "/system-diagnostics",
        "/nifi-api/access/config",
    )
    if target not in paths:
        return []

    body = base64.b64decode(response.body).decode("latin-1")
    status = re.match(r"\s*(\d+)", response.status)
    patterns = (
        r"supportsLogin",
        r"disconnectedNodeAcknowledged",
        r"(aggregate|node)Snapshots?",
    )
    if (
        status
        and int(status.group(1)) == 200
        and any(re.search(value, body) for value in patterns)
    ):
        return [finding("apache-nifi-detect")]
    return []
