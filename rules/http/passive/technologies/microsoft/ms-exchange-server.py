metadata = {
    "id": "ms-exchange-server",
    "about": "Microsoft Exchange Server Detect (passive Nuclei technology fingerprint)",
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
    if target != "/owa/auth/logon.aspx":
        return []

    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    versions = [
        base64.b64decode(item["value"]).decode("latin-1")
        for item in response.headers
        if item["name"].lower() == "x-owa-version"
    ]
    versions.extend(
        match.group(1) for match in re.finditer(r"/owa/auth/([0-9.]+)/", body)
    )
    titles = (
        "<title>Exchange Log In</title>",
        "<title>Microsoft Exchange - Outlook Web Access</title>",
    )
    if versions or any(value in body for value in titles):
        return [finding("ms-exchange-server", extracted=list(dict.fromkeys(versions)))]
    return []
