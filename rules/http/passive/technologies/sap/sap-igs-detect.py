metadata = {
    "id": "sap-igs-detect",
    "about": "SAP Internet Graphics Server (IGS) Detection (passive Nuclei technology fingerprint)",
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
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    servers = [
        base64.b64decode(header["value"]).decode("latin-1")
        for header in response.headers
        if header["name"].lower() == "server"
    ]
    headers = "\n".join(servers)
    status_match = re.match(r"\s*(\d+)", response.status)
    matched = (
        status_match
        and int(status_match.group(1)) == 200
        and "SAP IGS" in body
        and "is running" in body
        and "SAP Internet Graphics Server" in headers
    )
    return [finding("sap-igs-detect", extracted=servers)] if matched else []
