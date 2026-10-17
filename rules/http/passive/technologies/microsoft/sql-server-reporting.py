metadata = {
    "id": "sql-server-reporting",
    "about": "Microsoft SQL Server Reporting Services Detection (passive Nuclei technology fingerprint)",
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
    target = request.uri.split("://", 1)[-1]
    target = target[target.find("/") :] if "/" in target else "/"
    target = target.split("#", 1)[0].split("?", 1)[0] or "/"
    if target != "/Reports/Pages/Folder.aspx":
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    return (
        [finding("sql-server-reporting", extracted=[])]
        if "Report Manager" in body
        else []
    )
