metadata = {
    "id": "fullstory-rum-detect",
    "about": "FullStory RUM - Detect (passive Nuclei technology fingerprint)",
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


def scan(request: HttpRequest, response: HttpResponse):
    del request
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    markers = ("_fs_org", "_fs_host", "_fs_script", "_fs_namespace")
    return (
        [finding("fullstory-rum-detect")]
        if all(marker in body for marker in markers)
        else []
    )
