import base64
import re
from cruster import HttpRequest, HttpResponse, finding

metadata = {
    "id": "pixelfed-detect",
    "about": "Pixelfed - Detect (passive Nuclei technology fingerprint)",
    "authors": ["ProjectDiscovery contributors", "Cruster"],
    "references": [
        "https://github.com/projectdiscovery/nuclei-templates/tree/main/http/technologies"
    ],
    "tags": ["technology", "discovery", "nuclei", "passive"],
    "severity": "info",
    "type": "passive",
}


def scan(request: HttpRequest, response: HttpResponse):
    headers = []
    for item in response.headers:
        try:
            headers.append(
                item["name"] + ": " + base64.b64decode(item["value"]).decode("latin-1")
            )
        except Exception:
            headers.append(item["name"] + ": ")
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    status = re.match(r"\s*(\d+)", response.status)
    detected = (
        status is not None
        and int(status.group(1)) == 200
        and "window.pfl = {" in body
        and "/css/landing.css?id=" in body
        and re.search(r"(?mi)^set-cookie:\s*pxfs=", "\n".join(headers)) is not None
    )
    return [finding("pixelfed-detect", extracted=[])] if detected else []
