metadata = {
    "id": "aws-cloudfront-service",
    "about": "AWS Cloudfront service detection (passive Nuclei technology fingerprint)",
    "authors": ["ProjectDiscovery contributors", "Cruster"],
    "references": [
        "https://github.com/projectdiscovery/nuclei-templates/tree/main/http/technologies"
    ],
    "tags": ["technology", "discovery", "nuclei", "passive"],
    "severity": "info",
    "type": "passive"
}

import base64

from cruster import HttpRequest, HttpResponse, finding


def scan(request: HttpRequest, response: HttpResponse):
    cloudfront_values = (
        "hit from cloudfront",
        "refreshhit from cloudfront",
        "miss from cloudfront",
        "error from cloudfront",
    )

    for header in response.headers:
        if header["name"].lower() != "x-cache":
            continue
        try:
            value = base64.b64decode(header["value"]).decode("latin-1").lower()
        except Exception:
            continue
        if any(marker in value for marker in cloudfront_values):
            return [finding("aws-cloudfront-service")]

    return []
