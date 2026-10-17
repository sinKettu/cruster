metadata = {
    "id": "intel-amt-detect",
    "about": "Intel Active Management Technology Server Detection (passive Nuclei technology fingerprint)",
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
    target = request.uri.split("://", 1)[-1]
    target = target[target.find("/") :] if "/" in target else "/"
    target = target.split("#", 1)[0].split("?", 1)[0] or "/"
    if target != "/":
        return []

    status = re.match(r"\s*(\d+)", response.status)
    if not status or int(status.group(1)) != 200:
        return []

    headers = []
    for item in response.headers:
        try:
            value = base64.b64decode(item["value"]).decode("latin-1")
        except Exception:
            value = ""
        headers.append(item["name"] + ": " + value)
    header = "\n".join(headers)
    lowered_header = header.lower()
    if (
        "intel" not in lowered_header
        and "active management technology" not in lowered_header
    ):
        return []

    extracted = [
        match.group(1)
        for match in re.finditer(
            r"(?i)Intel\(R\)\s*Active\s*Management\s*Technology\s*([0-9.]+)",
            header,
        )
    ]
    return [finding("intel-amt-detect", extracted=extracted)]
