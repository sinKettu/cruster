metadata = {
    "id": "glpi-status-page",
    "about": "GLPI Status Page - Detect (passive Nuclei technology fingerprint)",
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
    if target.split("#", 1)[0].split("?", 1)[0] != "/status.php":
        return []
    status = re.match(r"\s*(\d+)", response.status)
    if not status or int(status.group(1)) != 200:
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []
    markers = ("GLPI_DB_OK", "GLPI_SESSION_DIR_OK", "GLPI_OK")
    return [finding("glpi-status-page")] if any(x in body for x in markers) else []
