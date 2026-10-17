metadata = {
    "id": "concrete5-detect",
    "about": "Concrete5 Detection (passive Nuclei technology fingerprint)",
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
    if not status or int(status.group(1)) != 200:
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    lower_body = body.lower()
    if (
        "powered by concrete5" not in lower_body
        and "x-powered-by-concrete5" not in lower_body
    ):
        return []
    extracted = []
    for pattern in (r"Concrete5 ([0-9.]+)", r"Powered by Concrete5 ([0-9.]+)"):
        extracted.extend(match.group(0) for match in re.finditer(pattern, body))
    for pattern in (r"concrete5-site-id=([a-z0-9]+)", r"concrete5\.org/([^/]+)"):
        extracted.extend(match.group(0) for match in re.finditer(pattern, body))
    return [finding("concrete5-detect", extracted=list(dict.fromkeys(extracted)))]
