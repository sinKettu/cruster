metadata = {
    "id": "cmsimple-detect",
    "about": "CMSimple - Detect (passive Nuclei technology fingerprint)",
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
    matched = re.search(
        r'(?i)content=["\']CMSimple(?:["\'\s>])', body
    ) is not None or any(
        word in body
        for word in (
            "cmsimpleLocatorElement",
            "Welcome_to_CMSimple",
            "templates/__cmsimple_",
        )
    )
    if not matched:
        return []
    extracted = [
        match.group(1)
        for match in re.finditer(r'(?i)content=["\']CMSimple ([0-9.]+)', body)
    ]
    return [finding("cmsimple-detect", extracted=list(dict.fromkeys(extracted)))]
