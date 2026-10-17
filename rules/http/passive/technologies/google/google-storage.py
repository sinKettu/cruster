metadata = {
    "id": "gstorage-detect",
    "about": "Google Bucket detection (passive Nuclei technology fingerprint)",
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
    if names & {"x-goog-metageneration", "x-goog-generation"}:
        return [finding("gstorage-detect")]
    return []
