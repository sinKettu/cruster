metadata = {
    "id": "google-bucket-service",
    "about": "Detect websites using Google bucket storage (passive Nuclei technology fingerprint)",
    "authors": ["ProjectDiscovery contributors", "Cruster"],
    "references": [
        "https://github.com/projectdiscovery/nuclei-templates/tree/main/http/technologies"
    ],
    "tags": ["technology", "discovery", "nuclei", "passive"],
    "severity": "info",
    "type": "passive",
}

from cruster import HttpRequest, HttpResponse, finding


def scan(request: HttpRequest, response: HttpResponse):
    del request
    names = {item["name"].lower() for item in response.headers}
    markers = {
        "x-goog-component-count",
        "x-goog-expiration",
        "x-goog-generation",
        "x-goog-metageneration",
        "x-goog-stored-content-encoding",
        "x-goog-stored-content-length",
        "x-guploader-uploadid",
    }
    if names & markers:
        return [finding("google-bucket-service")]
    return []
