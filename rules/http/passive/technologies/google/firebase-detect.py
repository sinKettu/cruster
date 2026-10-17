metadata = {
    "id": "firebase-detect",
    "about": "firebase detect (passive Nuclei technology fingerprint)",
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
    target = target.split("#", 1)[0] or "/"
    body = base64.b64decode(response.body).decode("latin-1")
    if (
        target == "/.settings/rules.json?auth=FIREBASE_SECRET"
        and "Could not parse auth token" in body
    ):
        return [finding("firebase-detect")]
    return []
