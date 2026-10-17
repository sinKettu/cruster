metadata = {
    "id": "apache-activemq-artemis-detect",
    "about": "Apache ActiveMQ Artemis - Detection (passive Nuclei technology fingerprint)",
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
    if path != "/console/hawtconfig.json":
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    if not any(
        word in body
        for word in ("img/activemq.png", "ActiveMQ Artemis", "Apache Artemis")
    ):
        return []
    match = re.search(r'"productInfo":.*"value":\s*"(.*)"', body)
    return [
        finding(
            "apache-activemq-artemis-detect",
            extracted=[match.group(1)] if match else [],
        )
    ]
