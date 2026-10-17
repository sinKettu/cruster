metadata = {
    "id": "solarassistant-detect",
    "about": "SolarAssistant - Detect (passive Nuclei technology fingerprint)",
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
    target = request.uri.split("://", 1)[-1]
    target = target[target.find("/") :] if "/" in target else "/"
    target = target.split("#", 1)[0].split("?", 1)[0] or "/"
    if target != "/sign_in":
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
    matched = (
        status is not None
        and int(status.group(1)) == 200
        and "<title>SolarAssistant</title>" in body
        and "phx-track-static" in body
        and "_solar_assistant_key=" in headers
    )
    return [finding("solarassistant-detect", extracted=[])] if matched else []
