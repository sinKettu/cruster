metadata = {
    "id": "raygun-rum-detect",
    "about": "Raygun RUM - Detect (passive Nuclei technology fingerprint)",
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
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""

    header_lines = []
    for header in response.headers:
        try:
            value = base64.b64decode(header["value"]).decode("latin-1")
        except Exception:
            value = ""
        header_lines.append(header["name"] + ": " + value)

    all_headers = "\n".join(header_lines)
    if "text/html" not in all_headers:
        return []

    if "markdown-body" in body or "blob-code" in body:
        return []

    raygun_patterns = (
        r"(?m)^[^/\n]*rg4js\s*\(\s*['\"]apiKey['\"]",
        r"(?m)^[^/\n]*rg4js\s*\(\s*['\"]enableCrashReporting['\"]\s*,\s*(?:true|!)",
        r"(?m)^[^/\n]*rg4js\s*\(\s*['\"]enableRealUserMonitoring['\"]\s*,\s*true",
    )
    if not any(re.search(pattern, body) for pattern in raygun_patterns):
        return []

    return [finding("raygun-rum-detect", extracted=[])]
