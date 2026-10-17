metadata = {
    "id": "couchbase-sync-gateway",
    "about": "Couchbase Sync Gateway (passive Nuclei technology fingerprint)",
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
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    if "Couchbase Sync Gateway" not in body:
        return []
    has_json = False
    for item in response.headers:
        try:
            value = base64.b64decode(item["value"]).decode("latin-1")
        except Exception:
            value = ""
        if "application/json" in item["name"] + ": " + value:
            has_json = True
            break
    if not has_json:
        return []
    extracted = [
        match.group(1)
        for match in re.finditer(
            r'version":"Couchbase Sync Gateway\/([0-9.(;a-z)]+)', body
        )
    ]
    return [finding("couchbase-sync-gateway", extracted=extracted)]
