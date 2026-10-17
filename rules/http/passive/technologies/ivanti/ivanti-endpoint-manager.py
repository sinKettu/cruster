metadata = {
    "id": "ivanti-endpoint-manager",
    "about": "Ivanti Endpoint Manager - Detect (passive Nuclei technology fingerprint)",
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
    if target not in {"/", "/ldlogon.dll", "/core/login", "/webaccess"}:
        return []
    body = base64.b64decode(response.body).decode("latin-1")
    headers = "\n".join(
        item["name"] + ": " + base64.b64decode(item["value"]).decode("latin-1")
        for item in response.headers
    )
    body_words = (
        "Ivanti Endpoint Manager",
        "LANDesk Management",
        "Ivanti(R) Cloud Services",
        "Ivanti Cloud Services Appliance",
        "LANDesk(R) Cloud Services Appliance",
        "LANDesk(R) Management",
    )
    body_lower, headers_lower = body.lower(), headers.lower()
    matched = (
        any(word.lower() in body_lower for word in body_words)
        or any(word in headers_lower for word in ("landesk", "ivanti epm"))
        or re.search(r"(?i)landdesk.*management", headers + "\n\n" + body)
        or re.search(r"(?i)ivanti.*endpoint\s*manager", headers + "\n\n" + body)
    )
    return [finding("ivanti-endpoint-manager")] if matched else []
