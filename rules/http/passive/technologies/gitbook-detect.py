metadata = {
    "id": "gitbook-detect",
    "about": "GitBook Detect (passive Nuclei technology fingerprint)",
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
        return []
    words = (
        "gitbook.page.hasChanged" in body
        and '<meta name="generator" content="GitBook' in body
    )
    title = (
        re.search(
            r"<title>.*[Gg]it[bB]ook</title>|<title>.*· [Gg]it[bB]ook.*</title>", body
        )
        is not None
    )
    if not words and not title:
        return []
    extracted = []
    for pattern in (
        r'<meta name="generator" content="GitBook ([0-9.]+)">',
        r'(?m)<meta name=generator content="GitBook ([0-9.]+)">',
    ):
        extracted.extend(match.group(1) for match in re.finditer(pattern, body))
    return [finding("gitbook-detect", extracted=list(dict.fromkeys(extracted)))]
