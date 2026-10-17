metadata = {
    "id": "vertigis-detect",
    "about": "VertiGIS - Detect (passive Nuclei technology fingerprint)",
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
    paths = [
        "/login",
        "/GeoManLogin.aspx",
        "/FM/GeoManLogin.aspx",
        "/GEBman/GeoManLogin.aspx",
        "/Geoportal/synserver",
        "/vertigisstudio/web/designer/locales/en/translations.json",
        "/vertigisstudio/search/designer/locales/en/translations.json",
        "/vertigisstudio/mobile/designer/locales/en/translations.json",
        "/vertigisstudio/accesscontrol/locales/en/translations.json",
    ]
    if not any(request.uri.split("?", 1)[0].endswith(path) for path in paths):
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1").lower()
    except Exception:
        return []
    markers = [
        "<title>vertigis",
        "welcome to vertigis",
        "vertigis fm - login",
        "<title>weboffice landing page",
        "vertigis studio app designer",
        "vertigis studio web designer",
        "vertigis studio access control",
    ]
    if not response.status.startswith("200"):
        return []
    if not any(marker in body for marker in markers):
        return []
    return [finding("vertigis-detect")]
