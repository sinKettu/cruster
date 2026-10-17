metadata = {
    "id": "orthanc-detect",
    "about": "Orthanc - Detect (passive Nuclei technology fingerprint)",
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
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    status = re.match(r"\s*(\d+)", response.status)
    script = 'src="explorer.js"' in body or 'src="../plugins/explorer.js"' in body
    if (
        status
        and int(status.group(1)) == 200
        and "<title>Orthanc Explorer</title>" in body
        and script
    ):
        return [finding("orthanc-detect", extracted=[])]
    return []
