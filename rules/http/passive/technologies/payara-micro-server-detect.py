import base64
import re
from cruster import HttpRequest, HttpResponse, finding

metadata = {
    "id": "payara-micro-detect",
    "about": "Payara Micro Detect (passive Nuclei technology fingerprint)",
    "authors": ["ProjectDiscovery contributors", "Cruster"],
    "references": [
        "https://github.com/projectdiscovery/nuclei-templates/tree/main/http/technologies",
    ],
    "tags": ["technology", "discovery", "nuclei", "passive"],
    "severity": "info",
    "type": "passive",
}


def scan(request: HttpRequest, response: HttpResponse):
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    status = re.match(r"\s*(\d+)", response.status)
    detected = (
        status is not None
        and int(status.group(1)) == 404
        and "Payara Micro #badassfish - Error report" in body
    )
    return [finding("payara-micro-detect", extracted=[])] if detected else []
