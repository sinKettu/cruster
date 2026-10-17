metadata = {
    "id": "tor-socks-proxy",
    "about": "Detect tor SOCKS proxy (passive Nuclei technology fingerprint)",
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


def scan(_request: HttpRequest, response: HttpResponse):
    status_code = response.status.lstrip().split(" ", 1)[0]
    if status_code != "501":
        return []

    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []

    indicators = (
        "This is a SOCKS Proxy",
        "HTTPTunnelPort",
        "SOCKSPort",
    )
    if not all(indicator in body for indicator in indicators):
        return []
    return [finding("tor-socks-proxy")]
