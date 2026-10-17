metadata = {
    "id": "akamai-bot-manager-detect",
    "about": "Akamai Bot Manager Protection - Detect (passive Nuclei technology fingerprint)",
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
    del request
    headers = []
    for item in response.headers:
        try:
            value = base64.b64decode(item["value"]).decode("latin-1")
        except Exception:
            value = ""
        headers.append(item["name"] + ": " + value)
    header = "\n".join(headers)
    patterns = (
        r"Set-Cookie:\s*_abck=",
        r"Set-Cookie:\s*bm_sv=",
        r"Set-Cookie:\s*ak_bmsc=",
        r"Set-Cookie:\s*bm_sz=",
        r"Set-Cookie:\s*bm_mi=",
    )
    if any(re.search(pattern, header) for pattern in patterns):
        return [finding("akamai-bot-manager-detect")]
    return []
