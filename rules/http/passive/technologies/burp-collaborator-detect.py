metadata = {
    "id": "burp-collaborator-detect",
    "about": "Burp Collaborator Server - Detect (passive Nuclei technology fingerprint)",
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
    status = re.match(r"\s*(\d+)", response.status)
    if not status or int(status.group(1)) != 200:
        return []

    headers = []
    for item in response.headers:
        try:
            value = base64.b64decode(item["value"]).decode("latin-1")
        except Exception:
            value = ""
        headers.append(item["name"] + ": " + value)
    header = "\n".join(headers)
    if "Server: Burp Collaborator https://burpcollaborator.net/" not in header:
        return []

    extracted = [
        match.group(1)
        for match in re.finditer(r"(?:X-(Collaborator-Version: [0-9]))", header)
    ]
    return [finding("burp-collaborator-detect", extracted=extracted)]
