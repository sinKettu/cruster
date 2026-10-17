metadata = {
    "id": "websphere-detect",
    "about": "IBM WebSphere Application Server (passive Nuclei technology fingerprint)",
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


def scan(_request: HttpRequest, response: HttpResponse):
    headers = []
    for item in response.headers:
        try:
            value = base64.b64decode(item["value"]).decode("latin-1")
        except Exception:
            value = ""
        headers.append(item["name"] + ": " + value)
    header_text = "\n".join(headers)
    if re.search(r"(?i)WebSphere Application Server", header_text) is None:
        return []
    pattern = r"(?i)WebSphere Application Server/([0-9.]+)"
    extracted = [match.group(1) for match in re.finditer(pattern, header_text)]
    return [finding("websphere-detect", extracted=extracted)]
