metadata = {
    "id": "drupal-eol",
    "about": "Drupal End-of-Life - Detect (passive Nuclei technology fingerprint)",
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
    target = request.uri.split("://", 1)[-1]
    target = target[target.find("/") :] if "/" in target else "/"
    target = target.split("#", 1)[0].split("?", 1)[0] or "/"
    if target not in {"/", "/core/CHANGELOG.txt", "/CHANGELOG.txt"}:
        return []
    body = base64.b64decode(response.body).decode("latin-1")
    headers = "\n".join(
        item["name"] + ": " + base64.b64decode(item["value"]).decode("latin-1")
        for item in response.headers
    )
    full_response = headers + "\n\n" + body
    patterns = (
        r"(?i)Drupal\s+([0-9]+\.[0-9]+(?:\.[0-9]+)?),\s+[0-9]{4}-",
        r'(?i)content=["\x27]?Drupal\s+([0-9]+(?:\.[0-9]+)*)',
        r"(?i)X-Generator:\s*Drupal\s+([0-9]+(?:\.[0-9]+)*)",
    )
    versions = []
    for pattern in patterns:
        versions.extend(match.group(1) for match in re.finditer(pattern, full_response))
    if versions and tuple(map(int, versions[0].split("."))) < (10, 0, 0):
        return [finding("drupal-eol", extracted=list(dict.fromkeys(versions)))]
    return []
