metadata = {
    "id": "browserless-swagger-detect",
    "about": "Browserless API Swagger - Detect (passive Nuclei technology fingerprint)",
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
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""

    status = re.match(r"\s*(\d+)", response.status)
    if not status or int(status.group(1)) != 200:
        return []

    target = request.uri.split("://", 1)[-1]
    target = target[target.find("/") :] if "/" in target else "/"
    target = target.split("#", 1)[0].split("?", 1)[0] or "/"
    if target == "/docs/swagger-ui-init.js":
        matched = (
            "browserless/chrome API" in body
            and "browserless Swagger documentation" in body
        )
    elif target == "/docs":
        matched = "<title>Swagger UI" in body
    else:
        matched = False

    if not matched:
        return []
    return [finding("browserless-swagger-detect", extracted=[])]
