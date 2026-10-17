metadata = {
    "id": "craftercms-detect",
    "about": "CrafterCMS - Detect (passive Nuclei technology fingerprint)",
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
    if "CrafterCMS" not in "\n".join(headers):
        return []
    extracted = [
        match.group(1)
        for match in re.finditer(r"Copyright\s+\(C\)\s+([0-9-]+)\s+Crafter", body)
    ]
    return [finding("craftercms-detect", extracted=extracted)]
