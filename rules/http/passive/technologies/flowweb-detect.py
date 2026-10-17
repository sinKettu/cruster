metadata = {
    "id": "flowweb-detect",
    "about": "FlowWeb Web Server - Detection (passive Nuclei technology fingerprint)",
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
    del request
    headers = []
    for item in response.headers:
        try:
            value = base64.b64decode(item["value"]).decode("latin-1")
        except Exception:
            value = ""
        headers.append(f'{item["name"]}: {value}')
    header = "\n".join(headers)
    if not re.search(r"FlowWeb/[0-9.]+", header, re.IGNORECASE):
        return []
    evidence = [match.group(1) for match in re.finditer(r"FlowWeb/([0-9.]+)", header)]
    return [finding("flowweb-detect", extracted=evidence)]
