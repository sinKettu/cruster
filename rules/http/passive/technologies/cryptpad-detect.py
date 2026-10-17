metadata = {
    "id": "cryptpad-detect",
    "about": "CryptPad - Detect (passive Nuclei technology fingerprint)",
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
    status = re.match(r"\s*(\d+)", response.status)
    if target != "/" or not status or int(status.group(1)) != 200:
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    markers = (
        'data-localization="main_title">CryptPad:',
        'data-bootload="/customize/template.js"',
        'data-main="/common/boot.js',
    )
    return (
        [finding("cryptpad-detect")]
        if all(marker in body for marker in markers)
        else []
    )
