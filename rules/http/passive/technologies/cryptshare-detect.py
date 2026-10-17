metadata = {
    "id": "cryptshare-detect",
    "about": "Pointsharp Cryptshare - Detect (passive Nuclei technology fingerprint)",
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
    target = target.split("#", 1)[0] or "/"
    status = re.match(r"\s*(\d+)", response.status)
    if target != "/Start?0" or not status or int(status.group(1)) != 200:
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    if 'name="generator" content="Cryptshare' not in body:
        return []
    extracted = [
        match.group(1)
        for match in re.finditer(
            r"<span>([\d\.]+)</span>\n<span>licensed for</span>", body
        )
    ]
    return [finding("cryptshare-detect", extracted=extracted)]
