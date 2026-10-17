metadata = {
    "id": "confluence-detect",
    "about": "Confluence Detection (passive Nuclei technology fingerprint)",
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
    headers = []
    for item in response.headers:
        try:
            value = base64.b64decode(item["value"]).decode("latin-1")
        except Exception:
            value = ""
        headers.append(item["name"] + ": " + value)
    if (
        "-confluence-" not in "\n".join(headers).lower()
        and "confluence-base-url" not in body.lower()
    ):
        return []
    extracted = []
    patterns = (
        r'<meta name="ajs-version-number" content="(.*)">',
        r"Atlassian Confluence ([a-z0-9-._]+)",
        r'<meta id="confluence-base-url" name="confluence-base-url" content="https://(.*)">',
        r'<meta name="ajs-base-url" content="https://(.*)">',
        r'<meta name="ajs-server-name" content="(.*)">',
    )
    for pattern in patterns:
        extracted.extend(match.group(1) for match in re.finditer(pattern, body))
    return [finding("confluence-detect", extracted=list(dict.fromkeys(extracted)))]
