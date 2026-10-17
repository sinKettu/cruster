metadata = {
    "id": "zimbra-detect",
    "about": "Zimbra Detect (passive Nuclei technology fingerprint)",
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
    if "/" in target:
        target = target[target.find("/") :]
    else:
        target = "/"
    target = target.split("?", 1)[0].split("#", 1)[0]
    if target != "/js/zimbraMail/share/model/ZmSettings.js":
        return []

    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []

    headers = []
    for item in response.headers:
        try:
            value = base64.b64decode(item["value"]).decode("latin-1")
        except Exception:
            continue
        headers.append(item["name"] + ": " + value)

    if "Zimbra Collaboration Suite Web Client" not in body:
        return []
    if "application/x-javascript" not in "\n".join(headers):
        return []

    pattern = (
        r"CLIENT_VERSION\",\t\t\t\t\t\{type:ZmSetting.T_CONFIG, defaultValue:\"(.*?)\""
    )
    versions = [match.group(1) for match in re.finditer(pattern, body)]
    return [finding("zimbra-detect", extracted=versions)]
