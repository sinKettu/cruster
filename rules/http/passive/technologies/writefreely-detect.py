metadata = {
    "id": "writefreely-detect",
    "about": "WriteFreely - Detect (passive Nuclei technology fingerprint)",
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
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []
    headers = []
    for item in response.headers:
        try:
            value = base64.b64decode(item["value"]).decode("latin-1")
        except Exception:
            value = ""
        headers.append(item["name"] + ": " + value)
    header_text = "\n".join(headers)
    if not response.status.startswith("200"):
        return []
    if re.search(r"(?mi)^set-cookie:\s*wfu=", header_text) is None:
        return []
    if re.search(r"""(?i)href=["']/?css/write(?:3)?\.css""", body) is None:
        return []
    return [finding("writefreely-detect")]
