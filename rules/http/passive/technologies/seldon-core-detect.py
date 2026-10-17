metadata = {
    "id": "seldon-core-detect",
    "about": "Seldon Core - Detect (passive Nuclei technology fingerprint)",
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
    uri_without_scheme = request.uri.split("://", 1)[-1]
    if "/" in uri_without_scheme:
        target = uri_without_scheme[uri_without_scheme.find("/") :]
    else:
        target = "/"
    target = target.split("#", 1)[0].split("?", 1)[0] or "/"
    if target != "/":
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    header_lines = []
    for header in response.headers:
        try:
            value = base64.b64decode(header["value"]).decode("latin-1")
        except Exception:
            value = ""
        header_lines.append(header["name"] + ": " + value)
    headers = "\n".join(header_lines)
    if (
        re.search(r'"name":\s*"seldon-pipelinegateway"', body) is None
        or "seldon-pipelinegateway" not in headers
    ):
        return []
    return [finding("seldon-core-detect", extracted=[])]
