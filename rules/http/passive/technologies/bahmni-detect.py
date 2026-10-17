metadata = {
    "id": "bahmni-detect",
    "about": "Bahmni - Detect (passive Nuclei technology fingerprint)",
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
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []

    detected = (
        status
        and int(status.group(1)) == 200
        and re.search(r"(?i)<title>Bahmni(?: Home)?</title>", body) is not None
        and (
            'ng-app="bahmni.home"' in body
            or "/bahmni/images/bahmniLogoFull.png" in body
        )
    )
    return [finding("bahmni-detect")] if detected else []
