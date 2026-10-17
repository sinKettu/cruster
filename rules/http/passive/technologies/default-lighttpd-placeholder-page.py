metadata = {
    "id": "lighttpd-placeholder-page",
    "about": "Lighttpd Placeholder Page (passive Nuclei technology fingerprint)",
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
    detected = (
        status
        and int(status.group(1)) == 200
        and "If you find a bug in this Lighttpd package, or in Lighttpd itself" in body
        and "<h1> Placeholder page </h1>" in body
    )
    return [finding("lighttpd-placeholder-page")] if detected else []
