metadata = {
    "id": "jway-products-detect",
    "about": "JWay Products - Detect (passive Nuclei technology fingerprint)",
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
    if target != "/generatedHomepage.html":
        return []

    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""

    status_match = re.match(r"\s*(\d+)", response.status)
    if not status_match or int(status_match.group(1)) != 200:
        return []
    lowered_body = body.lower()
    if not any(
        word in lowered_body
        for word in (
            "jway.footer.url.help",
            "jway.footer.url.contact",
            "jway.footer.url.legal",
        )
    ):
        return []

    extracted = []
    extracted.extend(
        match.group(1)
        for match in re.finditer(r"Produced\s+by\s+Form([a-z]+)", body, re.IGNORECASE)
    )
    extracted.extend(
        match.group(1) for match in re.finditer(r"Form(?:[a-zA-Z]+)\s+v([0-9.]+)", body)
    )
    return [finding("jway-products-detect", extracted=list(dict.fromkeys(extracted)))]
