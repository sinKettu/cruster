metadata = {
    "id": "graylog-api-exposure",
    "about": "Graylog REST API Endpoints - Exposure (passive Nuclei technology fingerprint)",
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
    status_code = int(status.group(1)) if status else 0
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
        headers.append(item["name"] + ": " + value)
    header = "\n".join(headers)

    success = (
        status_code == 200
        and any(
            marker in header for marker in ("X-Graylog-Node-Id", "Graylog", "graylog")
        )
        and any(
            marker in body for marker in ("X-Graylog-Node-Id", "Graylog", "graylog")
        )
        and "swagger" in body
    )
    if success:
        return [finding("graylog-api-exposure")]

    unauthorized = status_code == 401 and (
        "X-Graylog-Node-Id" in header or "Graylog Server" in header
    )
    if unauthorized:
        return [
            finding("graylog-api-exposure", extracted=["unauthorized-graylog-header"])
        ]
    return []
