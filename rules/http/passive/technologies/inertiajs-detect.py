metadata = {
    "id": "inertiajs-detect",
    "about": "Inertia.js - Detect (passive Nuclei technology fingerprint)",
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

    patterns = (
        (
            r"data-page=\"[^\"]*&quot;component&quot;[^\"]*&quot;props&quot;[^\"]*&quot;url&quot;",
            body,
        ),
        (r"data-page='[^']*\"component\":[^']*\"props\":[^']*\"url\":", body),
        (
            r"<script[^>]*data-page=[\"'][^>]*>[^<]*\"component\":[^<]*\"props\":[^<]*\"url\":",
            body,
        ),
        (r"(?im)^vary:[^\r\n]*x-inertia", header),
    )
    if not any(re.search(pattern, value) for pattern, value in patterns):
        return []
    return [finding("inertiajs-detect")]
