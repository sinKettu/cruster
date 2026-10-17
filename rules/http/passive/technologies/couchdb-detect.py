metadata = {
    "id": "couchdb-detect",
    "about": "CouchDB - Detect (passive Nuclei technology fingerprint)",
    "authors": ["ProjectDiscovery contributors", "Cruster"],
    "references": [
        "https://github.com/projectdiscovery/nuclei-templates/tree/main/http/technologies"
    ],
    "tags": ["technology", "discovery", "nuclei", "passive"],
    "severity": "info",
    "type": "passive",
}
import base64
import re

from cruster import HttpRequest, HttpResponse, finding


def scan(request: HttpRequest, response: HttpResponse):
    status = re.match(r"\s*(\d+)", response.status)
    if not status or int(status.group(1)) != 200:
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    if '"couchdb":' not in body or '"vendor":' not in body:
        return []
    content_types = []
    for item in response.headers:
        if item["name"].lower().replace("-", "_") != "content_type":
            continue
        try:
            content_types.append(base64.b64decode(item["value"]).decode("latin-1"))
        except Exception:
            continue
    if not any("application/json" in value for value in content_types):
        return []
    extracted = [
        match.group(1)
        for match in re.finditer(r'"version":"([0-9]+\.[0-9]+\.[0-9]+)"', body)
    ]
    return [finding("couchdb-detect", extracted=extracted)]
