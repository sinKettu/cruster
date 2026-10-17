metadata = {
    "id": "ibm-sterling-detect",
    "about": "IBM Sterling File Gateway Detect (passive Nuclei technology fingerprint)",
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
    return (
        [finding("ibm-sterling-detect", extracted=[])]
        if s
        and int(s.group(1)) == 200
        and "<title>Welcome to IBM Sterling File Gateway" in b
        else []
    )
