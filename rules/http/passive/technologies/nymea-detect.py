metadata = {
    "id": "nymea-detect",
    "about": "nymea - Detect (passive Nuclei technology fingerprint)",
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
    headers = "\n".join(
        item["name"] + ": " + base64.b64decode(item["value"]).decode("latin-1")
        for item in response.headers
    )
    status = re.match(r"\s*(\d+)", response.status)
    if (
        status
        and int(status.group(1)) == 404
        and re.search(r"(?mi)^Server:\s*nymea/[0-9]", headers)
    ):
        return [finding("nymea-detect")]
    return []
