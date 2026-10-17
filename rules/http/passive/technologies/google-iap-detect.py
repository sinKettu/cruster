metadata = {
    "id": "google-iap-detect",
    "about": "Google Identity-Aware Proxy (IAP) - Detect (passive Nuclei technology fingerprint)",
    "authors": ["ProjectDiscovery contributors", "Cruster"],
    "references": [
        "https://github.com/projectdiscovery/nuclei-templates/tree/main/http/technologies"
    ],
    "tags": ["technology", "discovery", "nuclei", "passive"],
    "severity": "info",
    "type": "passive"
}

from cruster import HttpRequest, HttpResponse, finding


def scan(request: HttpRequest, response: HttpResponse):
    del request
    for header in response.headers:
        if header["name"].lower() == "x-goog-iap-generated-response":
            return [finding("google-iap-detect")]
    return []
