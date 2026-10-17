metadata = {
    "id": "sap-netweaver-detect",
    "about": "SAP NetWeaver ICM - Detect (passive Nuclei technology fingerprint)",
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
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    servers = [
        base64.b64decode(header["value"]).decode("latin-1")
        for header in response.headers
        if header["name"].lower() == "server"
    ]
    headers = "\n".join(
        header["name"] + ": " + base64.b64decode(header["value"]).decode("latin-1")
        for header in response.headers
    )
    header_match = (
        "sap-server:" in headers.lower()
        or "sap netweaver application server" in headers.lower()
    )
    body_match = any(
        value in body
        for value in (
            "SAP NetWeaver Application Server",
            "SAP NetWeaver Developer Studio",
            "SAP Management Console",
        )
    )
    return (
        [finding("sap-netweaver-detect", extracted=servers)]
        if header_match or body_match
        else []
    )
