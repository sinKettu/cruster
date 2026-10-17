metadata = {
    "id": "opencart-detect",
    "about": "OpenCart Detection (passive Nuclei technology fingerprint)",
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
    del request
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
    status = re.match(r"\s*(\d+)", response.status)
    response_text = ("\n".join(headers) + "\n\n" + body).lower()
    markers = ("powered by opencart", "x-powered-by-opencart", "opencart store")
    if (
        not status
        or int(status.group(1)) != 200
        or not any(marker in response_text for marker in markers)
    ):
        return []
    extracted = []
    for pattern in (
        r"OpenCart ([0-9.]+)",
        r"Powered by OpenCart ([0-9.]+)",
        r"opencart-store-id=([a-z0-9]+)",
        r"oc-admin/([^/]+)",
    ):
        extracted.extend(re.findall(pattern, body))
    return [finding("opencart-detect", extracted=extracted)]
