metadata = {
    "id": "apache-tapestry-detect",
    "about": "Apache Tapestry Framework detect (passive Nuclei technology fingerprint)",
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
    if (
        'content="Apache Tapestry Framework' not in body
        and 'content="Tapestry Application Framework' not in body
    ):
        return []
    values = []
    for pattern in (
        r"Tapestry Framework \(version ([0-9.]+)\)",
        r'Tapestry Application Framework, version ([0-9.-A-Z]+)T" \/>',
    ):
        match = re.search(pattern, body)
        if match:
            values.append(match.group(1))
    return [finding("apache-tapestry-detect", extracted=values)]
