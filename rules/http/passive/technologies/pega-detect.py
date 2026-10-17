import base64
import re

from cruster import HttpRequest, HttpResponse, finding

metadata = {
    "id": "pega-detect",
    "about": "Pega Infinity Detection (passive Nuclei technology fingerprint)",
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
    if target != "/prweb/PRRestService/unauthenticatedAPI/v1/docs":
        return []
    headers = []
    for item in response.headers:
        try:
            headers.append(
                item["name"] + ": " + base64.b64decode(item["value"]).decode("latin-1")
            )
        except Exception:
            headers.append(item["name"] + ": ")
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    status = re.match(r"\s*(\d+)", response.status)
    detected = (
        status is not None
        and int(status.group(1)) == 200
        and '"description" :"The Pega API' in body
        and "application/json" in "\n".join(headers)
    )
    return [finding("pega-detect", extracted=[])] if detected else []
