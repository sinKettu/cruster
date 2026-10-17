metadata = {
    "id": "confluence-eol",
    "about": "Atlassian Confluence End-of-Life - Detect (passive Nuclei technology fingerprint)",
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
    body = base64.b64decode(response.body).decode("latin-1")
    patterns = (
        r'Atlassian Confluence</a>.*?<span[^>]*id=["\']footer-build-information["\'][^>]*>([0-9.]+)',
        r'<meta[^>]*name=["\']ajs-version-number["\'][^>]*content=["\']([0-9.]+)["\']',
    )
    versions = []
    for pattern in patterns:
        versions.extend(match.group(1) for match in re.finditer(pattern, body))
    identified = re.search(r"(?i)Atlassian Confluence", body) or re.search(
        r'id=["\']footer-build-information["\']', body
    )
    if (
        versions
        and identified
        and tuple(map(int, versions[0].split("."))) <= (8, 8, 99)
    ):
        return [finding("confluence-eol", extracted=list(dict.fromkeys(versions)))]
    return []
