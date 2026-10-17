import base64
import re

from cruster import HttpRequest, HttpResponse, finding

metadata = {
    "id": "pgweb-detect",
    "about": "Pgweb - Detect (passive Nuclei technology fingerprint)",
    "authors": ["ProjectDiscovery contributors", "Cruster"],
    "references": [
        "https://github.com/projectdiscovery/nuclei-templates/tree/main/http/technologies"
    ],
    "tags": ["technology", "discovery", "nuclei", "passive"],
    "severity": "info",
    "type": "passive",
}


def scan(request: HttpRequest, response: HttpResponse):
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    status = re.match(r"\s*(\d+)", response.status)
    signatures = (
        "<title>pgweb</title>",
        'src="static/js/ace-pgsql.js"',
        'id="table_connection"',
    )
    detected = (
        status is not None
        and int(status.group(1)) == 200
        and all(value in body for value in signatures)
    )
    return [finding("pgweb-detect", extracted=[])] if detected else []
