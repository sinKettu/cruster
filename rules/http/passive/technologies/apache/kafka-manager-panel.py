metadata = {
    "id": "kafka-manager-panel",
    "about": "Kafka Manager Panel (passive Nuclei technology fingerprint)",
    "authors": ["ProjectDiscovery contributors", "Cruster"],
    "references": [
        "https://github.com/projectdiscovery/nuclei-templates/tree/main/http/technologies"
    ],
    "tags": ["technology", "discovery", "nuclei", "passive"],
    "severity": "info",
    "type": "passive",
}

import base64

from cruster import HttpRequest, HttpResponse, finding


def scan(request: HttpRequest, response: HttpResponse):
    del request
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""

    header_lines = []
    for header in response.headers:
        try:
            value = base64.b64decode(header["value"]).decode("latin-1")
        except Exception:
            continue
        header_lines.append(header["name"] + ": " + value)

    headers = "\n".join(header_lines)
    if "Kafka Manager" not in body and "Kafka-Manager" not in headers:
        return []

    return [finding("kafka-manager-panel")]
