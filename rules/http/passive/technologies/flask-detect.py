metadata = {
    "id": "flask-detect",
    "about": "Flask Detection (passive Nuclei technology fingerprint)",
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
    status = re.match(r"\s*(\d+)", response.status)
    if status is None or int(status.group(1)) != 200:
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []
    headers = []
    for header in response.headers:
        try:
            value = base64.b64decode(header["value"]).decode("latin-1")
        except Exception:
            value = ""
        headers.append(header["name"] + ": " + value)
    response_text = ("\n".join(headers) + "\n\n" + body).lower()
    markers = ("powered by flask", "flask.palletsprojects.com", "x-powered-by-flask")
    if not any(marker in response_text for marker in markers):
        return []
    extracted = []
    for pattern in (
        r"Flask ([0-9.]+)",
        r"Powered by Flask ([0-9.]+)",
        r"flask-site-id=([a-z0-9]+)",
        r"flask\.palletsprojects\.com/([^/]+)",
    ):
        extracted.extend(match.group(1) for match in re.finditer(pattern, body))
    return [finding("flask-detect", extracted=list(dict.fromkeys(extracted)))]
