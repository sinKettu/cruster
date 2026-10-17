metadata = {
    "id": "default-websphere-liberty",
    "about": "WebSphere Liberty Default Page (passive Nuclei technology fingerprint)",
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
    del request
    status = re.match(r"\s*(\d+)", response.status)
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    if not status or int(status.group(1)) != 200 or "WebSphere Liberty" not in body:
        return []
    evidence = re.search(r"(WebSphere Liberty).*?([a-zA-Z0-9.-]+)", body)
    return [
        finding(
            "default-websphere-liberty",
            extracted=[evidence.group(0)] if evidence else [],
        )
    ]
