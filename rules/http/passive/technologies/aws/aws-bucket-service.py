metadata = {
    "id": "aws-bucket-service",
    "about": "Detect websites using AWS bucket storage (passive Nuclei technology fingerprint)",
    "authors": ["ProjectDiscovery contributors", "Cruster"],
    "references": [
        "https://github.com/projectdiscovery/nuclei-templates/tree/main/http/technologies"
    ],
    "tags": ["technology", "discovery", "nuclei", "passive"],
    "severity": "info",
    "type": "passive",
}
import base64

from cruster import HttpRequest, HttpResponse, finding


def scan(request: HttpRequest, response: HttpResponse):
    del request
    headers = []
    for item in response.headers:
        try:
            value = base64.b64decode(item["value"]).decode("latin-1")
        except Exception:
            value = ""
        headers.append(item["name"] + ": " + value)

    header = "\n".join(headers).lower()
    aws_marker = any(
        marker in header
        for marker in (
            "x-amz-bucket",
            "x-amz-request",
            "x-amz-id",
            "amazons3",
        )
    )
    if aws_marker and "x-guploader-uploadid" not in header:
        return [finding("aws-bucket-service")]
    return []
