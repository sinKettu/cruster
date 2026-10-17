metadata = {
    "id": "basic-auth-detect",
    "about": "Basic Auth Detection (passive Nuclei technology fingerprint)",
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
    status = re.match(r"\s*(\d+)", response.status)
    headers = []
    for item in response.headers:
        try:
            value = base64.b64decode(item["value"]).decode("latin-1")
        except Exception:
            value = ""
        headers.append(item["name"] + ": " + value)

    header = "\r\n".join(headers) + "\r\n"
    if (
        not status
        or int(status.group(1)) != 401
        or re.search(r"(?i)www-authenticate: basic", header) is None
    ):
        return []

    evidence = [
        match.group(1)
        for match in re.finditer(r"(?i)www-authenticate: (basic.*)\r\n", header)
    ]
    return [finding("basic-auth-detect", extracted=evidence)]
