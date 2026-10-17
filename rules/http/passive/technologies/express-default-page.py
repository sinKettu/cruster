metadata = {
    "id": "express-default-page",
    "about": "Express Default Page (passive Nuclei technology fingerprint)",
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
    headers = []
    for item in response.headers:
        try:
            value = base64.b64decode(item["value"]).decode("latin-1")
        except Exception:
            value = ""
        headers.append(f'{item["name"]}: {value}')
    detected = (
        "<title>Express</title>" in body
        and "Welcome to Express" in body
        and "text/html" in "\n".join(headers)
    )
    return (
        [finding("express-default-page")]
        if status and int(status.group(1)) == 200 and detected
        else []
    )
