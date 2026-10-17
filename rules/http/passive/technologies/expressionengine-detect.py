metadata = {
    "id": "expressionengine-detect",
    "about": "ExpressionEngine Detection (passive Nuclei technology fingerprint)",
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
        body = ""
    headers = []
    for item in response.headers:
        try:
            value = base64.b64decode(item["value"]).decode("latin-1")
        except Exception:
            value = ""
        headers.append(f'{item["name"]}: {value}')
    complete = "\n".join(headers) + "\n\n" + body
    markers = (
        "ExpressionEngine",
        "Powered by ExpressionEngine",
        "expressionengine.com",
        "X-Powered-By-ExpressionEngine",
    )
    if not any(marker.lower() in complete.lower() for marker in markers):
        return []
    evidence = []
    for pattern in (
        r"ExpressionEngine ([0-9.]+)",
        r"Powered by ExpressionEngine ([0-9.]+)",
        r"expressionengine-site-id=([a-z0-9]+)",
        r"expressionengine\.com/([^/]+)",
    ):
        evidence.extend(match.group(0) for match in re.finditer(pattern, body))
    return [finding("expressionengine-detect", extracted=list(dict.fromkeys(evidence)))]
