import base64
import re

from cruster import HttpRequest, HttpResponse, finding

metadata = {
    "id": "php-detect",
    "about": "PHP Detect (passive Nuclei technology fingerprint)",
    "authors": ["ProjectDiscovery contributors", "Cruster"],
    "references": [
        "https://github.com/projectdiscovery/nuclei-templates/tree/main/http/technologies",
    ],
    "tags": ["technology", "discovery", "nuclei", "passive"],
    "severity": "info",
    "type": "passive",
}


def scan(request: HttpRequest, response: HttpResponse):
    headers = []
    for item in response.headers:
        try:
            value = base64.b64decode(item["value"]).decode("latin-1")
        except Exception:
            value = ""
        headers.append(item["name"] + ": " + value)
    header = "\n".join(headers)
    versions = [
        match.group(1) for match in re.finditer(r"X-Powered-By: PHP/([0-9.]+)", header)
    ]
    status = re.match(r"\s*(\d+)", response.status)
    detected = status is not None and int(status.group(1)) == 200 and "PHP" in header
    return [finding("php-detect", extracted=versions)] if detected else []
