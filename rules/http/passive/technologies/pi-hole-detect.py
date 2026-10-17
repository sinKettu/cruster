metadata = {
    "id": "pi-hole-panel",
    "about": "Pi-hole Login Panel - Detect (passive Nuclei technology fingerprint)",
    "authors": ["ProjectDiscovery contributors", "Cruster"],
    "references": [
        "https://github.com/projectdiscovery/nuclei-templates/tree/main/http/technologies"
    ],
    "tags": ["technology", "discovery", "nuclei", "passive"],
    "severity": "info",
    "type": "passive"
}

import base64

from cruster import HttpRequest, HttpResponse, finding



SIGNATURES = (
    "<title>Pi-hole",
    "Pi-hole: Your black hole for Internet advertisements",
    "Pi-hole: A black hole for Internet advertisements",
    "<pre>sudo pihole -a -p</pre>",
)


def scan(request: HttpRequest, response: HttpResponse):
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    detected = all(
        value in body for value in ("Pi-hole", "Web Interface", "FTL")
    ) or any(value in body for value in SIGNATURES)
    return [finding("pi-hole-panel", extracted=[])] if detected else []
