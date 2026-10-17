metadata = {
    "id": "apache-tika-detect",
    "about": "Apache Tika - Detection (passive Nuclei technology fingerprint)",
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
    path = request.uri.split("://", 1)[-1]
    path = path[path.find("/") :] if "/" in path else "/"
    path = path.split("#", 1)[0].split("?", 1)[0] or "/"
    if path not in ("/", "/tika", "/version") or not response.status.startswith("200"):
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    headers = "\n".join(
        i["name"] + ": " + base64.b64decode(i["value"]).decode("latin-1")
        for i in response.headers
    )
    text = headers + "\n\n" + body
    if "Apache Tika" not in body or "apache.tika" not in body:
        return []
    match = re.search(r"Apache Tika [0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}", text)
    return [finding("apache-tika-detect", extracted=[match.group(0)] if match else [])]
