metadata = {
    "id": "docker-registry-ui-detect",
    "about": "Docker Registry UI - Detect (passive Nuclei technology fingerprint)",
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
    status = re.match(r"\s*(\d+)", response.status)
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    first = all(
        marker in body
        for marker in (
            "<title>Docker Registry UI</title>",
            'property="og:site_name" content="Docker Registry UI"',
            "docker-registry-ui.js",
        )
    )
    second = all(
        marker in body
        for marker in (
            "<title>Docker Registry UI</title>",
            "github.com/jc21/docker-registry-ui",
            "/images/favicons/browserconfig.xml",
        )
    )
    detected = status and int(status.group(1)) == 200 and (first or second)
    return [finding("docker-registry-ui-detect")] if detected else []
