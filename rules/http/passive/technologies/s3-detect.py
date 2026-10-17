metadata = {
    "id": "s3-detect",
    "about": "Detect Amazon-S3 Bucket (passive Nuclei technology fingerprint)",
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
    uri_without_scheme = request.uri.split("://", 1)[-1]
    if "/" in uri_without_scheme:
        request_target = uri_without_scheme[uri_without_scheme.find("/") :]
    else:
        request_target = "/"
    request_target = request_target.split("#", 1)[0]
    request_target = request_target.split("?", 1)[0] or "/"
    if request_target != "/%c0":
        return []

    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""

    header_lines = []
    server_values = []
    for header in response.headers:
        try:
            value = base64.b64decode(header["value"]).decode("latin-1")
        except Exception:
            value = ""
        header_lines.append(header["name"] + ": " + value)
        if header["name"].lower() == "server":
            server_values.append(value)

    headers = "\n".join(header_lines)
    body_match = re.search(r"(?:InvalidURI|InvalidArgument|NoSuchBucket)", body)
    matched = (
        body_match is not None
        or "amazons3" in "\n".join(server_values).lower()
        or "x-amz-id" in headers.lower()
        or "x-amz-request-id" in headers.lower()
    )
    if not matched:
        return []

    return [finding("s3-detect", extracted=[])]
