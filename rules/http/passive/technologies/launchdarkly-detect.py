metadata = {
    "id": "launchdarkly-detect",
    "about": "LaunchDarkly - Detect (passive Nuclei technology fingerprint)",
    "authors": ["ProjectDiscovery contributors", "Cruster"],
    "references": [
        "https://github.com/projectdiscovery/nuclei-templates/tree/main/http/technologies"
    ],
    "tags": ["technology", "discovery", "nuclei", "passive"],
    "severity": "info",
    "type": "passive",
}

import base64

from cruster import HttpRequest, HttpResponse, finding


def scan(request: HttpRequest, response: HttpResponse):
    del request
    body = base64.b64decode(response.body).decode("latin-1")
    markers = ("LDClient.initialize", "ldclient.min.js", "launchdarkly-js-client-sdk")
    if any(marker in body for marker in markers):
        return [finding("launchdarkly-detect")]
    return []
