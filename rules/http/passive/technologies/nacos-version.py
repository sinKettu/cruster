metadata = {
    "id": "nacos-version",
    "about": "Nacos - Detect (passive Nuclei technology fingerprint)",
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
    target = target.split("#", 1)[0]
    paths = (
        "/v1/console/server/state?accessToken=&username=",
        "/nacos/v1/console/server/state?accessToken=&username=",
    )
    if target not in paths:
        return []

    body = base64.b64decode(response.body).decode("latin-1")
    content_types = [
        base64.b64decode(item["value"]).decode("latin-1")
        for item in response.headers
        if item["name"].lower() == "content-type"
    ]
    status = re.match(r"\s*(\d+)", response.status)
    versions = re.findall(r'"version":"(\d+\.\d+\.\d+)"', body)
    if (
        status
        and int(status.group(1)) == 200
        and versions
        and any(re.search(r"application/json", value) for value in content_types)
    ):
        full_matches = re.findall(r'"version":"\d+\.\d+\.\d+"', body)
        return [finding("nacos-version", extracted=full_matches)]
    return []
