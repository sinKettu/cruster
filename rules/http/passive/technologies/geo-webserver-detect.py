metadata = {
    "id": "geo-webserver-detect",
    "about": "GeoWebServer Detector Panel - Detect (passive Nuclei technology fingerprint)",
    "authors": ["ProjectDiscovery contributors", "Cruster"],
    "references": [
        "https://github.com/projectdiscovery/nuclei-templates/tree/main/http/technologies"
    ],
    "tags": ["technology", "discovery", "nuclei", "passive"],
    "severity": "info",
    "type": "passive"
}
import base64
import re
from cruster import HttpRequest, HttpResponse, finding


def scan(request: HttpRequest, response: HttpResponse):
    del request
    status = re.match(r"\s*(\d+)", response.status)
    if not status or int(status.group(1)) != 200:
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []
    if "<TITLE>GeoWebServer</TITLE>" not in body:
        return []
    extracted = []
    for header in response.headers:
        try:
            line = (
                header["name"]
                + ": "
                + base64.b64decode(header["value"]).decode("latin-1")
            )
        except Exception:
            continue
        match = re.search(r"Server: GeoWebServer ([0-9.]+)", line)
        if match:
            extracted.append(match.group(1))
    return [finding("geo-webserver-detect", extracted=list(dict.fromkeys(extracted)))]
