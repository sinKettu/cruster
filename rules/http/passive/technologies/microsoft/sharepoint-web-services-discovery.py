metadata = {
    "id": "sharepoint-web-services-discovery",
    "about": "Microsoft SharePoint - Web Services Discovery (passive Nuclei technology fingerprint)",
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
    if target != "/_vti_bin/spdisco.aspx":
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1").lower()
        content_types = "\n".join(
            base64.b64decode(item["value"]).decode("latin-1")
            for item in response.headers
            if item["name"].lower() == "content-type"
        )
    except Exception:
        body = ""
        content_types = ""
    status = re.match(r"\s*(\d+)", response.status)
    matched = (
        status is not None
        and int(status.group(1)) == 200
        and "<discovery" in body
        and "<contractref" in body
        and "text/xml" in content_types
    )
    return (
        [finding("sharepoint-web-services-discovery", extracted=[])] if matched else []
    )
