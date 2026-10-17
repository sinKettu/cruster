metadata = {
    "id": "ibm-http-server",
    "about": "Default IBM HTTP Server (passive Nuclei technology fingerprint)",
    "authors": ["ProjectDiscovery contributors", "Cruster"],
    "references": [
        "https://github.com/projectdiscovery/nuclei-templates/tree/main/http/technologies"
    ],
    "tags": ["technology", "discovery", "nuclei", "passive"],
    "severity": "info",
    "type": "passive",
}
import base64, re
from cruster import HttpRequest, HttpResponse, finding


def scan(request: HttpRequest, response: HttpResponse):
    try:
        b = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        b = ""
    s = re.match(r"\s*(\d+)", response.status)
    v = [m.group(0) for m in re.finditer(r"IBM HTTP Server ([0-9.]+)", b)]
    return (
        [finding("ibm-http-server", extracted=v)]
        if s and int(s.group(1)) == 200 and "<title>IBM HTTP Server</title>" in b
        else []
    )
