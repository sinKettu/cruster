metadata = {
    "id": "aspnet-version-detect",
    "about": "AspNet Version Disclosure - Detect (passive Nuclei technology fingerprint)",
    "authors": ["ProjectDiscovery contributors", "Cruster"],
    "references": [
        "https://github.com/projectdiscovery/nuclei-templates/tree/main/http/technologies"
    ],
    "tags": ["technology", "discovery", "nuclei", "passive"],
    "severity": "info",
    "type": "passive"
}
import base64, re
from cruster import HttpRequest, HttpResponse, finding


def scan(request: HttpRequest, response: HttpResponse):
    values = []
    lines = []
    for h in response.headers:
        v = base64.b64decode(h["value"]).decode("latin-1")
        lines.append(h["name"] + ": " + v)
        if h["name"].lower().replace("-", "_") == "x_aspnet_version":
            values.append(v)
    header = "\n".join(lines)
    return (
        [finding("aspnet-version-detect", extracted=values)]
        if "x-aspnet-version" in header.lower()
        and re.search(r"(?i)^X-AspNet-Version:\s*$", header, re.M) is None
        else []
    )
