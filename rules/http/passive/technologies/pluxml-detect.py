import base64
import re

from cruster import HttpRequest, HttpResponse, finding

metadata = {
    "id": "pluxml-detect",
    "about": "PluXml - Detect (passive Nuclei technology fingerprint)",
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
    stylesheet = re.search(
        r"""href=["'][^"']*/themes/[^/]+/css/plucss(?:\.min)?\.css\?v=[0-9.]+""", body
    )
    feed = re.search(r"""href=["'][^"']*/feed\.php\?rss/commentaires["']""", body)
    detected = (
        status is not None
        and int(status.group(1)) == 200
        and stylesheet is not None
        and feed is not None
    )
    return [finding("pluxml-detect", extracted=[])] if detected else []
