metadata = {
    "id": "simplesamlphp-detect",
    "about": "SimpleSAMLphp - Detect (passive Nuclei technology fingerprint)",
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
    paths = (
        "/simplesaml/module.php/core/frontpage_welcome.php",
        "/module.php/core/frontpage_welcome.php",
    )
    if target not in paths:
        return []
    status = re.match(r"\s*(\d+)", response.status)
    if status is None or int(status.group(1)) != 200:
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1").lower()
    except Exception:
        body = ""
    signatures = (
        "you have successfully installed simplesamlphp",
        "simplesamlphp installation page",
        "/module.php/core/login-admin.php",
    )
    if not any(signature in body for signature in signatures):
        return []
    return [finding("simplesamlphp-detect", extracted=[])]
