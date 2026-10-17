metadata = {
    "id": "detect-sentry",
    "about": "Detect Sentry Instance (passive Nuclei technology fingerprint)",
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
    if not status or int(status.group(1)) != 200 or "sentry.init" not in body.lower():
        return []
    evidence = re.findall(r"https://[0-9a-f]*@[a-z0-9]+\.[a-z.]+.?[0-9]+", body)
    return [finding("detect-sentry", extracted=evidence)]
