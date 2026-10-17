metadata = {
    "id": "bugzilla-detect",
    "about": "Bugzilla - Detect (passive Nuclei technology fingerprint)",
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
    status = re.match(r"\s*(\d+)", response.status)
    if not status or int(status.group(1)) != 200:
        return []

    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    if not (
        "<title>Bugzilla Main" in body
        or 'title="Bugzilla' in body
        or 'href="https://bugzilla.readthedocs.org/' in body
    ):
        return []

    extracted = [
        match.group(1)
        for match in re.finditer(
            r'class="header_addl_info">version ([^<]+)</span>',
            body,
        )
    ]
    return [finding("bugzilla-detect", extracted=list(dict.fromkeys(extracted)))]
