metadata = {
    "id": "winstone-detect",
    "about": "Winstone Servlet Engine (passive Nuclei technology fingerprint)",
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


def scan(_request: HttpRequest, response: HttpResponse):
    headers = []
    for item in response.headers:
        try:
            value = base64.b64decode(item["value"]).decode("latin-1")
        except Exception:
            value = ""
        headers.append(item["name"] + ": " + value)
    header_text = "\n".join(headers)
    detection_patterns = [
        r"(?i)Server:\s*Winstone Servlet Engine\s*v?[0-9.]+",
        r"(?i)Winstone/[0-9.]+",
        r"(?i)X-Powered-By:\s*Servlet/[0-9.]+\s*\(Winstone/[0-9.]+\)",
    ]
    if not any(re.search(pattern, header_text) for pattern in detection_patterns):
        return []
    extraction_patterns = [
        r"(?i)Winstone(?: Servlet Engine)?\s*v?[0-9.]+",
        r"(?i)Winstone/[0-9.]+",
    ]
    extracted = []
    for pattern in extraction_patterns:
        extracted.extend(match.group(0) for match in re.finditer(pattern, header_text))
    return [finding("winstone-detect", extracted=extracted)]
