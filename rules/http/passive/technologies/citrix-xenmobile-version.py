metadata = {
    "id": "citrix-xenmobile-version",
    "about": "Citrix XenMobile Version - Detect (passive Nuclei technology fingerprint)",
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
    status = re.match(r"\s*(\d+)", response.status)
    status_code = int(status.group(1)) if status else 0
    headers = []
    for item in response.headers:
        try:
            value = base64.b64decode(item["value"]).decode("latin-1")
        except Exception:
            value = ""
        headers.append(item["name"] + ": " + value)
    if status_code == 302 and "Location: /zdm/login_xdm_uc.jsp" in "\n".join(headers):
        return [finding("citrix-xenmobile-version")]

    target = request.uri.split("://", 1)[-1]
    target = target[target.find("/") :] if "/" in target else "/"
    target = target.split("#", 1)[0].split("?", 1)[0] or "/"
    if target != "/js/app/init.js" or status_code != 200:
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    matches = list(re.finditer(r"v=([^\"]+)", body))
    if not matches:
        return []
    extracted = ["version"]
    extracted.extend(match.group(1) for match in matches)
    return [finding("citrix-xenmobile-version", extracted=extracted)]
