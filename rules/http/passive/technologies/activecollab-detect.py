metadata = {
    "id": "activecollab-detect",
    "about": "activecollab detect (passive Nuclei technology fingerprint)",
    "authors": ["ProjectDiscovery contributors", "Cruster"],
    "references": [
        "https://github.com/projectdiscovery/nuclei-templates/tree/main/http/technologies"
    ],
    "tags": ["technology", "discovery", "nuclei", "passive"],
    "severity": "info",
    "type": "passive",
}

import base64

from cruster import HttpRequest, HttpResponse, finding


def scan(request: HttpRequest, response: HttpResponse):
    try:
        body = base64.b64decode(response.body).decode("latin-1")
        status_code = int(response.status.split()[0])
    except Exception:
        return []
    markers = (
        "http://www.activecollab.com",
        '<form method="post" id="system_form_2" class="uniForm focusFirstField">',
    )
    if status_code != 200 or not any(marker in body for marker in markers):
        return []
    return [finding("activecollab-detect")]
