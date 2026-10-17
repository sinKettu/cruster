metadata = {
    "id": "ibm-decision-server-runtime",
    "about": "IBM Decision Server Runtime Panel- Detect (passive Nuclei technology fingerprint)",
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
    p = request.uri.split("://", 1)[-1]
    p = p[p.find("/") :] if "/" in p else "/"
    p = p.split("#", 1)[0].split("?", 1)[0] or "/"
    if p != "/DecisionService/":
        return []
    try:
        b = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        b = ""
    s = re.match(r"\s*(\d+)", response.status)
    v = [m.group(1) for m in re.finditer(r"Decision Server ([0-9.]+)", b)]
    return (
        [finding("ibm-decision-server-runtime", extracted=v)]
        if s
        and int(s.group(1)) == 200
        and "<title>Hosted Transparent Decision Service</title>" in b
        else []
    )
