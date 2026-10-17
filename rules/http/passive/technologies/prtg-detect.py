import base64
import re

from cruster import HttpRequest, HttpResponse, finding

metadata = {
    "id": "prtg-detect",
    "about": "Detect PRTG (passive Nuclei technology fingerprint)",
    "authors": ["ProjectDiscovery contributors", "Cruster"],
    "references": [
        "https://github.com/projectdiscovery/nuclei-templates/tree/main/http/technologies"
    ],
    "tags": ["technology", "discovery", "nuclei", "passive"],
    "severity": "info",
    "type": "passive",
}


def scan(request: HttpRequest, response: HttpResponse):
    target = request.uri.split("://", 1)[-1]
    target = target[target.find("/") :] if "/" in target else "/"
    target = target.split("#", 1)[0].split("?", 1)[0] or "/"
    if target not in ("/index.htm", "/prtg/index.htm", "/PRTG/index.htm"):
        return []
    headers = []
    servers = []
    for item in response.headers:
        try:
            value = base64.b64decode(item["value"]).decode("latin-1")
        except Exception:
            value = ""
        headers.append(item["name"] + ": " + value)
        if item["name"].lower() == "server":
            servers.append(value)
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    status = re.match(r"\s*(\d+)", response.status)
    detected = (
        status is not None
        and int(status.group(1)) == 200
        and "PRTG Network Monitor" in body
        and re.search(r"PRTG", "\n".join(headers)) is not None
    )
    return [finding("prtg-detect", extracted=servers)] if detected else []
