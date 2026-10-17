metadata = {
    "id": "firebase-auth-fingerprint",
    "about": "Firebase Authentication - Fingerprint (passive Nuclei technology fingerprint)",
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
    target = request.uri.split("://", 1)[-1]
    target = target[target.find("/") :] if "/" in target else "/"
    if target.split("?", 1)[0].split("#", 1)[0] != "/__/auth/handler.js":
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    if (
        "@license Firebase" not in body
        or "https://firebase.google.com/terms/" not in body
    ):
        return []
    pattern = r"Build: [0-9]+\.[0-9]+\.[0-9]+(-rc\.[0-9]+)?"
    return [finding("firebase-auth-fingerprint")] if re.search(pattern, body) else []
