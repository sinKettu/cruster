metadata = {
    "id": "bigip-detect",
    "about": "BIGIP - Detection (passive Nuclei technology fingerprint)",
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
    header = "\n".join(headers)

    if not ("BIGipServer" in header or "BigIP" in header or "F5 Networks." in body):
        return []

    extracted = [
        match.group(1)
        for match in re.finditer(
            r"(BIGipServer[a-z_\.\-\~0-9A-Z]*)=([!a-z0-9A-Z/.=]+)",
            header,
        )
    ]
    return [finding("bigip-detect", extracted=list(dict.fromkeys(extracted)))]
