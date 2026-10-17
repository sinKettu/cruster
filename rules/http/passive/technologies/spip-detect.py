metadata = {
    "id": "spip-detect",
    "about": "SPIP - Detect (passive Nuclei technology fingerprint)",
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
    try:
        headers = "\n".join(
            item["name"] + ": " + base64.b64decode(item["value"]).decode("latin-1")
            for item in response.headers
        )
    except Exception:
        headers = ""
    status = re.match(r"\s*(\d+)", response.status)
    matched = (
        status is not None
        and int(status.group(1)) == 200
        and (
            re.search(r"Composed-By: SPIP", headers)
            or re.search(r"X-Spip-Cache:", headers)
        )
    )
    if not matched:
        return []
    versions = [
        match.group(1)
        for match in re.finditer(
            r"Composed-By: SPIP ([0-9]{1,2}.[0-9]{1,2}.[0-9]{1,2}) @ www.spip.net",
            headers,
        )
    ]
    return [finding("spip-detect", extracted=versions)]
