metadata = {
    "id": "carestream-vue-detect",
    "about": "CARESTREAM Vue Motion Detector (passive Nuclei technology fingerprint)",
    "authors": ["ProjectDiscovery contributors", "Cruster"],
    "references": [
        "https://github.com/projectdiscovery/nuclei-templates/tree/main/http/technologies"
    ],
    "tags": ["technology", "discovery", "nuclei", "passive"],
    "severity": "info",
    "type": "passive"
}
import base64
import hashlib
import re

from cruster import HttpRequest, HttpResponse, finding


def scan(request: HttpRequest, response: HttpResponse):
    target = request.uri.split("://", 1)[-1]
    target = target[target.find("/") :] if "/" in target else "/"
    target = target.split("#", 1)[0].split("?", 1)[0] or "/"
    if target not in ("/portal/favicon.ico", "/portal/images/MyVue/MyVueHelp.png"):
        return []

    status = re.match(r"\s*(\d+)", response.status)
    if not status or int(status.group(1)) != 200:
        return []

    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    digest = hashlib.md5(body.encode("latin-1")).hexdigest()
    evidence = []
    if digest == "84054196570ecd7526894ccb6e7c5c6e":
        evidence.append("myvuehelp")
    if digest == "1c4b2c10acccc48852d12eddadaf7944":
        evidence.append("myvuehelp-favicon")
    if not evidence:
        return []
    return [finding("carestream-vue-detect", extracted=evidence)]
