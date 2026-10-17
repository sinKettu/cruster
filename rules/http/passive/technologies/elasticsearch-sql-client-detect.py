metadata = {
    "id": "elasticsearch-sql-client-detect",
    "about": "Elasticsearch - SQL Client Detection (passive Nuclei technology fingerprint)",
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
    status = re.match(r"\s*(\d+)", response.status)
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    return (
        [finding("elasticsearch-sql-client-detect")]
        if status
        and int(status.group(1)) == 200
        and "<title>Elasticsearch-sql client</title>" in body
        else []
    )
