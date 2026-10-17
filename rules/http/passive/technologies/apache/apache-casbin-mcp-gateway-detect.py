metadata = {
    "id": "apache-casbin-mcp-gateway-detect",
    "about": "Apache Casbin MCP Gateway - Detection (passive Nuclei technology fingerprint)",
    "authors": ["ProjectDiscovery contributors", "Cruster"],
    "references": [
        "https://github.com/projectdiscovery/nuclei-templates/tree/main/http/technologies"
    ],
    "tags": ["technology", "discovery", "nuclei", "passive"],
    "severity": "info",
    "type": "passive"
}

import base64
import json

from cruster import HttpRequest, HttpResponse, finding


def scan(request: HttpRequest, response: HttpResponse):
    path = request.uri.split("://", 1)[-1]
    path = path[path.find("/") :] if "/" in path else "/"
    path = path.split("#", 1)[0].split("?", 1)[0] or "/"
    if path != "/health" or not response.status.startswith("200"):
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    if '"service": "mcp-gateway"' not in body:
        return []
    try:
        value = json.loads(body).get("version")
    except Exception:
        value = None
    return [
        finding(
            "apache-casbin-mcp-gateway-detect",
            extracted=[str(value)] if value is not None else [],
        )
    ]
