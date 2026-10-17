metadata = {
    "id": "default-glassfish-server-page",
    "about": "GlassFish Server Default Page (passive Nuclei technology fingerprint)",
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
        body = base64.b64decode(response.body).decode("latin-1").lower()
    except Exception:
        body = ""
    markers = (
        "glassfish server - server running",
        "glassfish server with premier support",
        "<b>glassfish server</b>",
        "glassfish server installation directory",
    )
    if (
        not status
        or int(status.group(1)) != 200
        or not any(marker in body for marker in markers)
    ):
        return []
    header = []
    for item in response.headers:
        try:
            value = base64.b64decode(item["value"]).decode("latin-1")
        except Exception:
            value = ""
        header.append(item["name"] + ": " + value)
    version = re.search(
        r"GlassFish\s+Server\s+([A-Za-z0-9\s.]+)(?:\n|\r)", "\n".join(header)
    )
    return [
        finding(
            "default-glassfish-server-page",
            extracted=[version.group(1)] if version else [],
        )
    ]
