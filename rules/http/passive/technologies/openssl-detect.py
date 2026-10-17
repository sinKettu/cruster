metadata = {
    "id": "openssl-detect",
    "about": "Openssl Detect (passive Nuclei technology fingerprint)",
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
    headers = []
    server = []
    for item in response.headers:
        try:
            value = base64.b64decode(item["value"]).decode("latin-1")
        except Exception:
            value = ""
        headers.append(item["name"] + ": " + value)
        if item["name"].lower() == "server":
            server.append(value)
    status = re.match(r"\s*(\d+)", response.status)
    if not status or int(status.group(1)) != 200 or "OpenSSL" not in "\n".join(server):
        return []
    versions = re.findall(r"OpenSSL\/([0-9.a-z]+)", "\n".join(headers))
    return [finding("openssl-detect", extracted=versions)]
