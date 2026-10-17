metadata = {
    "id": "drupal-detect",
    "about": "Drupal - Detect (passive Nuclei technology fingerprint)",
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
    detected = (
        ("Initial release" in body and "Drupal 1.0.0" in body)
        or 'content="Drupal' in body
        or re.search(r"(?i)x-drupal|x-generator: drupal", header)
    )
    if not detected:
        return []
    version = re.search(r'class="site-version">([0-9.x-]+)', body)
    return [finding("drupal-detect", extracted=[version.group(1)] if version else [])]
