metadata = {
    "id": "phppgadmin-version",
    "about": "PhpPgAdmin Version Information - Detect (passive Nuclei technology fingerprint)",
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
    if target != "/phppgadmin/intro.php":
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    status = re.match(r"\s*(\d+)", response.status)
    versions = [match.group(0) for match in re.finditer(r"phpPgAdmin.*\(.*\)", body)]
    detected = (
        status is not None
        and int(status.group(1)) == 200
        and re.search(
            r'<span class="appname">phpPgAdmin</span> <span class="version">.*</span>',
            body,
        )
        is not None
    )
    return [finding("phppgadmin-version", extracted=versions)] if detected else []
