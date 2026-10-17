metadata = {
    "id": "litellm-swagger-detect",
    "about": "LiteLLM API - Swagger UI Detection (passive Nuclei technology fingerprint)",
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
    body = base64.b64decode(response.body).decode("latin-1")
    if (
        status
        and int(status.group(1)) == 200
        and "<title>LiteLLM API - Swagger UI" in body
    ):
        return [finding("litellm-swagger-detect")]
    return []
