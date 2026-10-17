import base64
import re

from cruster import HttpRequest, HttpResponse, finding

metadata = {
    "id": "pomerium-detect",
    "about": "Pomerium Detect (passive Nuclei technology fingerprint)",
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
    if target != "/.pomerium/index.js":
        return []
    content_type = ""
    for item in response.headers:
        if item["name"].lower() == "content-type":
            try:
                content_type += base64.b64decode(item["value"]).decode("latin-1")
            except Exception:
                pass
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    status = re.match(r"\s*(\d+)", response.status)
    detected = (
        status is not None
        and int(status.group(1)) == 200
        and "text/javascript" in content_type
        and "https://pomerium.com/docs" in body
        and "https://discuss.pomerium.com" in body
    )
    return [finding("pomerium-detect", extracted=[])] if detected else []
