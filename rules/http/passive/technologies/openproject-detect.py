metadata = {
    "id": "openproject-detect",
    "about": "OpenProject - Detect (passive Nuclei technology fingerprint)",
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
    del request
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    headers = []
    for item in response.headers:
        try:
            value = base64.b64decode(item["value"]).decode("latin-1")
        except Exception:
            value = ""
        headers.append(item["name"] + ": " + value)
    regex_hit = any(
        re.search(pattern, body)
        for pattern in (
            r".*OpenProject.Foundation.\(OPF\)",
            r"<generator uri=.*openproject.org.*>",
        )
    )
    word_hit = "OpenProject" in body and "instanceName" in body
    header_hit = "_open_project_session" in "\n".join(headers)
    if regex_hit or word_hit or header_hit:
        return [finding("openproject-detect", extracted=[])]
    return []
