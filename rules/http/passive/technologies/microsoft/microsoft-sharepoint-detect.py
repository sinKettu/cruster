metadata = {
    "id": "microsoft-sharepoint-detect",
    "about": "Microsoft SharePoint Detect (passive Nuclei technology fingerprint)",
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
    values = [
        base64.b64decode(item["value"]).decode("latin-1")
        for item in response.headers
        if item["name"].lower() == "microsoftsharepointteamservices"
    ]
    if values:
        return [finding("microsoft-sharepoint-detect", extracted=values)]
    return []
