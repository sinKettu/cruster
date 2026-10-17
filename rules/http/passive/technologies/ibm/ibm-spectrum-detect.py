metadata = {
    "id": "ibm-spectrum-detect",
    "about": "IBM Spectrum - Detect (passive Nuclei technology fingerprint)",
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
    if p not in ("/", "/BACLIENT", "/JNLP"):
        return []
    try:
        b = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        b = ""
    s = re.match(r"\s*(\d+)", response.status)
    v = []
    for pattern in (
        r'(?i)"guiVersion"\s*:\s*"([0-9.\-]+)"',
        r'(?i)name="?version"?\s+value="?([0-9.\-]+)"?',
    ):
        v.extend(m.group(1) for m in re.finditer(pattern, b))
    return (
        [finding("ibm-spectrum-detect", extracted=v)]
        if s
        and int(s.group(1)) == 200
        and "ibm spectrum" in b.lower()
        and "com.ibm." in b.lower()
        else []
    )
