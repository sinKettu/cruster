metadata = {
    "id": "oqtane-cms-db",
    "about": "Oqtane CMS Database - Detect (passive Nuclei technology fingerprint)",
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
    if path != "/api/database":
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    content_types = []
    for item in response.headers:
        if item["name"].lower().replace("-", "_") == "content_type":
            try:
                content_types.append(base64.b64decode(item["value"]).decode("latin-1"))
            except Exception:
                pass
    status = re.match(r"\s*(\d+)", response.status)
    markers = ('controlType"', "Oqtane", 'dbType"')
    if (
        not status
        or int(status.group(1)) != 200
        or "application/json" not in "\n".join(content_types)
        or not all(marker in body for marker in markers)
    ):
        return []
    return [finding("oqtane-cms-db", extracted=[])]
