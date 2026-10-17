metadata = {
    "id": "firebase-urls",
    "about": "Google Firebase DB URL Finder (passive Nuclei technology fingerprint)",
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
    del request
    body = base64.b64decode(response.body).decode("latin-1")
    if re.search(
        r".*?(f|F)(i|I)(r|R)(e|E)(b|B)(a|A)(s|S)(e|E)(i|I)(o|O)[.](c|C)(o|O)(m|M).*?",
        body,
    ):
        return [finding("firebase-urls")]
    return []
