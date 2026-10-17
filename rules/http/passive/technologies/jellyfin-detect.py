metadata = {
    "id": "jellyfin-detect",
    "about": "Jellyfin detected (passive Nuclei technology fingerprint)",
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


def _request_path(uri):
    value = uri.split("://", 1)[-1]
    value = value[value.find("/") :] if "/" in value else "/"
    return value.split("#", 1)[0].split("?", 1)[0] or "/"


def scan(request: HttpRequest, response: HttpResponse):
    paths = (
        "/home.html",
        "/web/home.html",
        "/index.html",
        "/web/index.html",
        "/web/manifest.json",
    )
    if _request_path(request.uri) not in paths:
        return []

    status = re.match(r"\s*(\d+)", response.status)
    if not status or int(status.group(1)) != 200:
        return []

    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""

    markers = (
        'name="application-name" content="Jellyfin"',
        'class="page homePage libraryPage allLibraryPage backdropPage pageWithAbsoluteTabs withTabs"',
        "The Free Software Media System",
    )
    if not any(marker in body for marker in markers):
        return []

    return [finding("jellyfin-detect")]
