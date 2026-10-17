metadata = {
    "id": "owncloud-status-page",
    "about": "Owncloud StatusPage detection (passive Nuclei technology fingerprint)",
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
    target = request.uri.split("://", 1)[-1]
    target = target[target.find("/") :] if "/" in target else "/"
    target = target.split("#", 1)[0].split("?", 1)[0]
    if target != "/status.php":
        return []

    body = base64.b64decode(response.body).decode("latin-1")
    indicators = ('"versionstring":', '"installed":', '"edition":')
    if all(indicator in body for indicator in indicators):
        return [finding("owncloud-status-page")]
    return []
