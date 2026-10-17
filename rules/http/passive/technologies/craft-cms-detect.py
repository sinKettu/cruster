metadata = {
    "id": "craft-cms-detect",
    "about": "Craft CMS Detect (passive Nuclei technology fingerprint)",
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
    matched = (
        "X-Powered-By: Craft CMS" in header
        or re.search(r"Set-Cookie: (Craft|CRAFT)", header) is not None
        or re.search(r"(?i)CRAFT_CSRF_TOKEN", body) is not None
    )
    return [finding("craft-cms-detect")] if matched else []
