metadata = {
    "id": "arcgis-detect",
    "about": "ArcGIS - Detect (passive Nuclei technology fingerprint)",
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
    target = target.split("?", 1)[0].split("#", 1)[0]
    paths = (
        "/portal/sharing/rest",
        "/portal/portalhelp/en/rest/",
        "/arcgis/rest/services",
        "/server/rest/services",
        "/arcgis/",
    )
    status = re.match(r"\s*(\d+)", response.status)
    if target not in paths or status is None or int(status.group(1)) != 200:
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []

    markers = (
        "<title>arcgis portal directory",
        "search arcgis enterprise help",
        "<title>arcgis</title>",
        "arcgis rest services directory",
        "<title>arcgis hub</title>",
    )
    if not any(marker in body.lower() for marker in markers):
        return []

    extracted = []
    patterns = (
        r"(?i)&copy;\s+([0-9]+)\s+Esri\.",
        r"<b>Current\s+Version:\s+</b>([0-9.]+)",
    )
    for pattern in patterns:
        extracted.extend(match.group(1) for match in re.finditer(pattern, body))

    return [finding("arcgis-detect", extracted=extracted)]
