metadata = {
    "id": "frappe-framework-detect",
    "about": "Frappe Framework - Detect (passive Nuclei technology fingerprint)",
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
    if not status or int(status.group(1)) != 200:
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1").lower()
    except Exception:
        body = ""
    markers = ("frappe.csrf_token", "frappe.boot", "frappe-web.bundle")
    return (
        [finding("frappe-framework-detect")]
        if any(marker in body for marker in markers)
        else []
    )
