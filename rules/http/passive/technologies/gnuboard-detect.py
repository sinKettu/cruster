metadata = {
    "id": "gnuboard-detect",
    "about": "Gnuboard CMS - Detect (passive Nuclei technology fingerprint)",
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
    if target.split("#", 1)[0].split("?", 1)[0] != "/LICENSE.txt":
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []
    if "GNU Board" not in body or "http://sir.kr" not in body:
        return []
    content_type = ""
    for header in response.headers:
        if header["name"].lower() == "content-type":
            try:
                content_type += base64.b64decode(header["value"]).decode("latin-1")
            except Exception:
                pass
    if "text/plain" not in content_type:
        return []
    extracted = [match.group(1) for match in re.finditer(r"\( GNU Board (.*) \)", body)]
    return [finding("gnuboard-detect", extracted=list(dict.fromkeys(extracted)))]
