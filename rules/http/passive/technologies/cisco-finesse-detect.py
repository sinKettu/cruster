metadata = {
    "id": "cisco-finesse-detect",
    "about": "Cisco Finesse - Detect (passive Nuclei technology fingerprint)",
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
    if target != "/desktop/assets/js/finesse.js":
        return []
    status = re.match(r"\s*(\d+)", response.status)
    if not status or int(status.group(1)) != 200:
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    if "Cisco Finesse" not in body:
        return []
    extracted = [
        match.group(1)
        for match in re.finditer(r"(?i)Version\s+([\d.]+[(]\d+[)])", body)
    ]
    return [finding("cisco-finesse-detect", extracted=extracted)]
