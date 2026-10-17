metadata = {
    "id": "sap-web-dispatcher-admin-portal",
    "about": "SAP Web Dispatcher admin portal detection (passive Nuclei technology fingerprint)",
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
    target = target.split("#", 1)[0].split("?", 1)[0] or "/"
    if target != "/sap/wdisp/admin/public/default.html":
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1")
        headers = "\n".join(
            item["name"] + ": " + base64.b64decode(item["value"]).decode("latin-1")
            for item in response.headers
        )
    except Exception:
        body = ""
        headers = ""
    status = re.match(r"\s*(\d+)", response.status)
    header_match = (
        'Basic realm="WEB ADMIN"' in headers
        or "SAP NetWeaver Application Server" in headers
    )
    body_match = "SAP Web Dispatcher" in body or "<title>Administration</title>" in body
    matched = (
        status is not None
        and int(status.group(1)) in (401, 200)
        and header_match
        and body_match
    )
    return [finding("sap-web-dispatcher-admin-portal", extracted=[])] if matched else []
