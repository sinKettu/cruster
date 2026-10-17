metadata = {
    "id": "jira-detect",
    "about": "Jira Detect (passive Nuclei technology fingerprint)",
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
    paths = ("/secure/Dashboard.jspa", "/jira/secure/Dashboard.jspa", "/login.jsp")
    if _request_path(request.uri) not in paths:
        return []

    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""

    if "Project Management Software" not in body:
        return []

    extracted = [
        match.group(1)
        for match in re.finditer(r'title="JiraVersion" value="([0-9.]+)', body)
    ]
    return [finding("jira-detect", extracted=extracted)]
