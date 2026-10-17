metadata = {
    "id": "infinispan-detect",
    "about": "Infinispan Console - Detection (passive Nuclei technology fingerprint)",
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
    target = target.split("#", 1)[0].split("?", 1)[0] or "/"
    if target not in ("/console/welcome", "/rest/v2/cache-managers/default"):
        return []

    status = re.match(r"\s*(\d+)", response.status)
    status_code = int(status.group(1)) if status else 0
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""

    headers = []
    for item in response.headers:
        try:
            value = base64.b64decode(item["value"]).decode("latin-1")
        except Exception:
            value = ""
        headers.append(item["name"] + ": " + value)
    header = "\n".join(headers).lower()

    console_markers = (
        "<title>Console</title>",
        "/console/main.bundle.js",
        "pf-m-redhat-font",
    )
    console_match = status_code == 200 and all(
        marker in body for marker in console_markers
    )
    api_match = (
        status_code == 401
        and "www-authenticate" in header
        and 'digest realm="default"' in header
    )
    if not console_match and not api_match:
        return []

    extracted = ["console"] if console_match else ["rest-api"]
    extracted.extend(
        match.group(1)
        for match in re.finditer(
            r"\"version\"\s*:\s*\"([0-9.]+(?:\.[A-Za-z0-9-]+)?)\"", body
        )
    )
    return [finding("infinispan-detect", extracted=list(dict.fromkeys(extracted)))]
