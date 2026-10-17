metadata = {
    "id": "sitecore-version",
    "about": "Sitecore version detection (passive Nuclei technology fingerprint)",
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
    if target != "/sitecore/shell/sitecore.version.xml":
        return []
    status = re.match(r"\s*(\d+)", response.status)
    if status is None or int(status.group(1)) != 200:
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    if "Sitecore Corporation" not in body:
        return []
    extracted = []
    for name in ("major", "minor", "build", "revision"):
        extracted.extend(re.findall(r"<" + name + r">([0-9]+)</" + name + r">", body))
    return [finding("sitecore-version", extracted=extracted)]
